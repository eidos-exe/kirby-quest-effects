#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank { Good, Great, Excellent }

#[derive(Clone,Copy)]
pub struct SpitRegistry {shots:[Option<(u32,u32)>;16],cursor:usize}
impl SpitRegistry {
    pub const fn new()->Self {Self{shots:[None;16],cursor:0}}
    pub fn record(&mut self,owner:u32,shot:u32) {
        if shot==u32::MAX || self.shots.contains(&Some((owner,shot))) {return;}
        self.shots[self.cursor]=Some((owner,shot));self.cursor=(self.cursor+1)%16;
    }
    pub fn owner(&self,shot:u32)->Option<u32> {self.shots.iter().flatten().find(|(_,s)|*s==shot).map(|(o,_)|*o)}
    pub fn reset(&mut self,owner:u32) {for shot in &mut self.shots {if shot.map(|s|s.0)==Some(owner) {*shot=None;}}}
}

#[derive(Clone,Copy)]
pub struct TechFeedback {active:bool}
impl TechFeedback {
    pub const fn new()->Self {Self{active:false}}
    pub fn observe(&mut self,rank:Option<Rank>)->Option<Rank> {
        let fresh=rank.is_some()&&!self.active;self.active=rank.is_some();if fresh {rank}else{None}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase { Startup, Active, Recovery, Unknown }

#[derive(Clone,Copy)]
pub struct CaptureSession {pub target:Option<u32>,inhale:bool}
impl CaptureSession {
    pub const fn new()->Self {Self{target:None,inhale:false}}
    pub fn begin(&mut self,target:u32,inhale:bool)->bool {
        if self.target.is_some() && (self.target==Some(target) || (self.inhale && inhale)) {return false;}
        self.target=Some(target);self.inhale=inhale;true
    }
    pub fn observe(&mut self,inhale_continues:bool,capture_link_valid:bool) {
        if (self.inhale && !inhale_continues) || (!self.inhale && !capture_link_valid) {*self=Self::new();}
    }
}

pub fn phase(frame: f32, first: f32, last: f32, active: bool, actionable: bool) -> Phase {
    if actionable { Phase::Unknown }
    else if active { Phase::Active }
    else if frame < first { Phase::Startup }
    else if frame > last { Phase::Recovery }
    else { Phase::Unknown }
}

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub kind: i32,
    pub motion: u64,
    pub status: i32,
    pub frame: f32,
    pub opponent: usize,
    pub object: usize,
    pub epoch: u64,
    pub phase: Phase,
    pub smash: bool,
    pub slow: bool,
    pub actionable: bool,
    pub armored: bool,
}

pub fn actionable(frame: f32, cancel: f32, end: f32, cancel_enabled: bool) -> bool {
    let limit=if cancel > 0.0 { cancel } else { end };
    cancel_enabled || (limit > 0.0 && frame >= limit)
}

// A collision can notify after the damage status replaces the move. Keep the
// last non-damage observation independently of percent bookkeeping.
#[derive(Clone, Copy)]
pub struct AttackHistory { previous: Option<Attack> }
impl AttackHistory {
    pub const fn new() -> Self { Self {previous:None} }
    pub fn observe(&mut self, current: Attack, damage_reaction: bool) {
        if !damage_reaction {self.previous=Some(current);}
    }
    pub fn before_damage(&self, object: usize) -> Option<Attack> {
        self.previous.filter(|a|a.object==object)
    }
}

// The unchanged suffix can locate the native collision function even when
// another Skyline plugin has already patched its first instructions.
pub const COLLISION_SIGNATURE: &[u8]=&[
    0xff,0x03,0x03,0xd1,0xe8,0x2b,0x00,0xfd,0xfc,0x6f,0x06,0xa9,0xfa,0x67,0x07,0xa9,
    0xf8,0x5f,0x08,0xa9,0xf6,0x57,0x09,0xa9,0xf4,0x4f,0x0a,0xa9,0xfd,0x7b,0x0b,0xa9,
    0xfd,0xc3,0x02,0x91,0xfb,0x03,0x00,0xaa];
pub fn collision_offset(text: &[u8]) -> Option<usize> {
    let suffix=&COLLISION_SIGNATURE[20..];
    let mut hits=text.windows(suffix.len()).enumerate().filter(|(_,b)|*b==suffix)
        .filter_map(|(offset,_)|offset.checked_sub(20)).filter(|offset|offset%4==0);
    let first=hits.next()?;
    if hits.next().is_some() {None} else {Some(first)}
}
pub fn collision_offset_for_version(text: &[u8],version: &str) -> Option<usize> {
    if let Some(offset)=collision_offset(text) {return Some(offset);}
    // The older instruction signature is not a reliable locator on these
    // supported game versions; use the version-specific fallback instead.
    let offset=0x67a7b0;
    if matches!(version,"13.0.3"|"13.0.4"|"13.0.5") && text.len()>offset+40 {
        let instruction=u32::from_le_bytes([text[offset],text[offset+1],text[offset+2],text[offset+3]]);
        let next=u32::from_le_bytes([text[offset+4],text[offset+5],text[offset+6],text[offset+7]]);
        let stack_entry=instruction&0xffc003ff==0xd10003ff;
        let direct_hook=instruction&0xfc000000==0x14000000;
        let literal_hook=instruction&0xff000000==0x58000000 && next&0xfffffc1f==0xd61f0000;
        if stack_entry || direct_hook || literal_hook {return Some(offset);}
    }
    None
}

#[derive(Clone, Copy)]
pub struct State {
    parried: Option<(usize, usize, u64)>,
    rewarded: [Option<(usize, u64)>; 8],
    stars: [Option<u32>;16],
    star_cursor: usize,
}

impl State {
    pub const fn new() -> Self { Self { parried: None, rewarded: [None; 8],stars:[None;16],star_cursor:0 } }

    pub fn projectile_star(&mut self, id:u32, inhaled_projectile:bool, opponent_hit:bool)->Option<Rank> {
        if !inhaled_projectile || !opponent_hit || self.stars.contains(&Some(id)) {return None;}
        self.stars[self.star_cursor]=Some(id);self.star_cursor=(self.star_cursor+1)%self.stars.len();
        Some(Rank::Excellent)
    }

    pub fn parry(&mut self, attack: Option<Attack>) -> Rank {
        self.parried = attack.filter(|a| a.opponent < 8)
            .map(|a| (a.opponent, a.object, a.epoch));
        Rank::Excellent
    }

    pub fn hit(&mut self, attack: Attack, applied_damage: bool) -> Option<Rank> {
        if attack.armored && attack.phase==Phase::Startup {return None;}
        if !applied_damage || attack.opponent >= 8 || attack.actionable { return None; }
        let key = (attack.object, attack.epoch);
        if self.rewarded[attack.opponent] == Some(key) { return None; }
        let followup = self.parried == Some((attack.opponent, attack.object, attack.epoch));
        let rank = if followup { Rank::Excellent }
            else if attack.phase == Phase::Startup || attack.phase == Phase::Recovery {
                if attack.smash || attack.slow { Rank::Good } else { Rank::Great }
            } else { return None; };
        self.rewarded[attack.opponent] = Some(key);
        if followup { self.parried = None; }
        Some(rank)
    }

    pub fn grab(&mut self, attack: Option<Attack>) -> Rank {
        if let Some(a)=attack {
            if a.opponent<8 {self.rewarded[a.opponent]=Some((a.object,a.epoch));}
            if self.parried.map(|x|x.0)==Some(a.opponent) {self.parried=None;}
            if !a.actionable && matches!(a.phase,Phase::Startup|Phase::Recovery) {return Rank::Great;}
        }
        Rank::Good
    }

    pub fn armor_break(&mut self, attack: Attack, interrupted: bool) -> Option<Rank> {
        if !interrupted || !attack.armored || attack.phase!=Phase::Startup || attack.opponent>=8 {return None;}
        let key=(attack.object,attack.epoch);
        if self.rewarded[attack.opponent]==Some(key) {return None;}
        self.rewarded[attack.opponent]=Some(key);
        Some(Rank::Great)
    }

    pub fn observe_opponent(&mut self, attack: Attack) {
        if self.parried.map(|x| x.0) == Some(attack.opponent)
            && (attack.actionable || self.parried != Some((attack.opponent, attack.object, attack.epoch))) {
            self.parried = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn spit_projectile_credit_is_owned_and_cleared_on_stock_reset() {let mut shots=SpitRegistry::new();shots.record(10,100);shots.record(20,200);assert_eq!(shots.owner(100),Some(10));assert_eq!(shots.owner(200),Some(20));assert_eq!(shots.owner(300),None);shots.reset(10);assert_eq!(shots.owner(100),None);assert_eq!(shots.owner(200),Some(20));}
    #[test] fn ground_tech_is_great_once_then_next_tech_can_reward() {let mut t=TechFeedback::new();assert_eq!(t.observe(Some(Rank::Great)),Some(Rank::Great));for _ in 0..30 {assert_eq!(t.observe(Some(Rank::Great)),None);}t.observe(None);assert_eq!(t.observe(Some(Rank::Great)),Some(Rank::Great));}
    #[test] fn wall_and_ceiling_techs_are_top_without_duplicate_wall_jump() {let mut t=TechFeedback::new();assert_eq!(t.observe(Some(Rank::Excellent)),Some(Rank::Excellent));assert_eq!(t.observe(Some(Rank::Excellent)),None);t.observe(None);assert_eq!(t.observe(Some(Rank::Excellent)),Some(Rank::Excellent));}
    #[test] fn missed_tech_and_ordinary_landing_stay_silent() {let mut t=TechFeedback::new();for _ in 0..30 {assert_eq!(t.observe(None),None);}}
    #[test] fn inhale_link_gaps_and_swallow_spit_transitions_do_not_repeat() {
        let mut c=CaptureSession::new();assert!(c.begin(44,true));
        for _ in 0..12 {c.observe(true,false);assert!(!c.begin(44,true));}
        c.observe(true,true);assert!(!c.begin(45,true));
    }
    #[test] fn next_inhale_can_reward_the_same_opponent_again() {
        let mut c=CaptureSession::new();assert!(c.begin(44,true));c.observe(false,false);assert!(c.begin(44,true));
    }
    #[test] fn normal_grabs_still_reset_on_release_and_ignore_pummels() {
        let mut c=CaptureSession::new();assert!(c.begin(44,false));c.observe(false,true);assert!(!c.begin(44,false));c.observe(false,false);assert!(c.begin(44,false));
    }
    fn attack(phase: Phase) -> Attack {
        Attack { kind: 0, motion: 0, status: 0, frame: 0.0, opponent: 1, object: 10, epoch: 1, phase, smash: false, slow: false, actionable: false, armored:false }
    }
    #[test] fn projectile_star_requires_a_successful_opponent_hit() {let mut s=State::new();assert_eq!(s.projectile_star(44,true,false),None);assert_eq!(s.projectile_star(44,true,true),Some(Rank::Excellent));}
    #[test] fn spitting_a_fighter_is_not_the_projectile_conversion_reward() {assert_eq!(State::new().projectile_star(44,false,true),None);}
    #[test] fn same_star_has_one_celebration_but_new_star_can_reward() {let mut s=State::new();assert_eq!(s.projectile_star(44,true,true),Some(Rank::Excellent));assert_eq!(s.projectile_star(44,true,true),None);assert_eq!(s.projectile_star(45,true,true),Some(Rank::Excellent));}
    #[test] fn ordinary_grab_is_good() {assert_eq!(State::new().grab(None),Rank::Good);assert_eq!(State::new().grab(Some(attack(Phase::Unknown))),Rank::Good);}
    #[test] fn startup_and_recovery_grabs_are_great_even_against_smashes() {
        for phase in [Phase::Startup,Phase::Recovery] {let mut a=attack(phase);a.smash=true;a.slow=true;assert_eq!(State::new().grab(Some(a)),Rank::Great);}
    }
    #[test] fn grab_after_recovery_is_only_good() {let mut a=attack(Phase::Recovery);a.actionable=true;assert_eq!(State::new().grab(Some(a)),Rank::Good);}
    #[test] fn grab_consumes_attack_reward_so_pummel_does_not_repeat() {let mut s=State::new();let a=attack(Phase::Startup);assert_eq!(s.grab(Some(a)),Rank::Great);assert_eq!(s.hit(a,true),None);}
    #[test] fn intact_armor_does_not_reward_damage_or_parry_followup() {let mut s=State::new();let mut a=attack(Phase::Startup);a.armored=true;s.parry(Some(a));assert_eq!(s.hit(a,true),None);assert_eq!(s.armor_break(a,false),None);}
    #[test] fn breaking_startup_armor_is_great_once() {let mut s=State::new();let mut a=attack(Phase::Startup);a.armored=true;a.slow=true;assert_eq!(s.armor_break(a,true),Some(Rank::Great));assert_eq!(s.armor_break(a,true),None);}
    #[test] fn inhale_or_grab_bypassing_startup_armor_is_great() {let mut a=attack(Phase::Startup);a.armored=true;a.slow=true;assert_eq!(State::new().grab(Some(a)),Rank::Great);}
    #[test] fn zero_cancel_marker_does_not_make_whole_smash_actionable() {
        assert!(!actionable(30.0,0.0,55.0,false));
        assert!(actionable(55.0,0.0,55.0,false));
        assert!(actionable(30.0,0.0,55.0,true));
    }
    #[test] fn collision_retains_smash_until_real_hit_without_percent_polling() {
        let mut history=AttackHistory::new();let mut prior=attack(Phase::Recovery);prior.smash=true;
        history.observe(prior,false);
        let reaction=attack(Phase::Unknown);
        for _ in 0..30 {history.observe(reaction,true);}
        let cached=history.before_damage(10).unwrap();
        let mut state=State::new();assert_eq!(state.hit(cached,true),Some(Rank::Good));
        assert_eq!(state.hit(cached,true),None);
    }
    #[test] fn recovery_ending_before_hit_remains_silent() {
        let mut history=AttackHistory::new();history.observe(attack(Phase::Recovery),false);
        let mut idle=attack(Phase::Unknown);idle.actionable=true;
        history.observe(idle,false);
        assert_eq!(State::new().hit(history.before_damage(10).unwrap(),true),None);
        assert!(history.before_damage(11).is_none());
    }
    #[test] fn original_collision_prologue_is_located() {
        let mut image=vec![0;64];image.extend_from_slice(COLLISION_SIGNATURE);
        assert_eq!(collision_offset(&image),Some(64));
    }
    #[test] fn already_hooked_collision_prologue_is_located() {
        let mut image=vec![0;64];image.extend_from_slice(COLLISION_SIGNATURE);
        image[64..84].fill(0xcc);
        assert_eq!(collision_offset(&image),Some(64));
    }
    #[test] fn ambiguous_and_missing_collision_bodies_are_rejected() {
        assert_eq!(collision_offset(&[0;100]),None);
        let mut image=COLLISION_SIGNATURE.to_vec();image.extend_from_slice(COLLISION_SIGNATURE);
        assert_eq!(collision_offset(&image),None);
    }
    #[test] fn supported_version_uses_working_mod_fallback_when_signature_changed() {
        let mut text=vec![0;0x67a7b0+64];text[0x67a7b0..0x67a7b4].copy_from_slice(&0x58000051u32.to_le_bytes());text[0x67a7b4..0x67a7b8].copy_from_slice(&0xd61f0220u32.to_le_bytes());
        assert_eq!(collision_offset(&text),None);
        assert_eq!(collision_offset_for_version(&text,"13.0.4"),Some(0x67a7b0));
        assert_eq!(collision_offset_for_version(&text,"13.0.3"),Some(0x67a7b0));
        assert_eq!(collision_offset_for_version(&text,"13.0.5"),Some(0x67a7b0));
        assert_eq!(collision_offset_for_version(&text,"unknown"),None);
        text[0x67a7b0..0x67a7b4].copy_from_slice(&0x12345678u32.to_le_bytes());
        assert_eq!(collision_offset_for_version(&text,"13.0.5"),None);
        text[0x67a7b0..0x67a7b4].fill(0);
        assert_eq!(collision_offset_for_version(&text,"13.0.4"),None);
    }
    #[test] fn gaps_between_multihits_are_not_recovery() {
        assert_eq!(phase(25.0,15.0,38.0,false,false),Phase::Unknown);
        assert_eq!(phase(39.0,15.0,38.0,false,false),Phase::Recovery);
    }
    #[test] fn actionability_overrides_remaining_animation() {
        assert_eq!(phase(40.0,15.0,38.0,false,true),Phase::Unknown);
    }
    #[test] fn native_active_hitboxes_override_reference_timing() {
        assert_eq!(phase(40.0,15.0,38.0,true,false),Phase::Active);
    }
    #[test] fn every_smash_is_good_in_startup_and_recovery() {
        for phase in [Phase::Startup, Phase::Recovery] {
            let mut s=State::new(); let mut a=attack(phase); a.smash=true;
            assert_eq!(s.hit(a,true),Some(Rank::Good));
        }
    }
    #[test] fn slow_non_smash_is_good() {
        let mut s=State::new();let mut a=attack(Phase::Startup);a.slow=true;
        assert_eq!(s.hit(a,true),Some(Rank::Good));
    }
    #[test] fn fast_counter_and_punish_are_great() {
        for phase in [Phase::Startup,Phase::Recovery] {
            assert_eq!(State::new().hit(attack(phase),true),Some(Rank::Great));
        }
    }
    #[test] fn active_and_unknown_are_silent() {
        for phase in [Phase::Active,Phase::Unknown] {
            assert_eq!(State::new().hit(attack(phase),true),None);
        }
    }
    #[test] fn ordinary_hit_and_shield_contact_are_silent() {
        let mut s=State::new();let mut a=attack(Phase::Unknown);a.actionable=true;
        assert_eq!(s.hit(a,true),None);
        assert_eq!(s.hit(attack(Phase::Recovery),false),None);
    }
    #[test] fn parry_alone_is_excellent_even_without_known_owner() {
        assert_eq!(State::new().parry(None),Rank::Excellent);
    }
    #[test] fn parry_and_followup_are_two_excellents() {
        let mut s=State::new();let mut a=attack(Phase::Recovery);a.smash=true;
        assert_eq!(s.parry(Some(a)),Rank::Excellent);
        assert_eq!(s.hit(a,true),Some(Rank::Excellent));
        assert_eq!(s.hit(a,true),None);
    }
    #[test] fn parry_followup_can_interrupt_remaining_multihit_activity() {
        let mut s=State::new();let a=attack(Phase::Active);
        assert_eq!(s.parry(Some(a)),Rank::Excellent);
        assert_eq!(s.hit(a,true),Some(Rank::Excellent));
    }
    #[test] fn other_opponent_does_not_receive_parry_credit() {
        let mut s=State::new();let a=attack(Phase::Recovery);s.parry(Some(a));
        let mut b=a;b.opponent=2;
        assert_eq!(s.hit(b,true),Some(Rank::Great));
        assert_eq!(s.hit(a,true),Some(Rank::Excellent));
    }
    #[test] fn parry_credit_expires_on_actionability() {
        let mut s=State::new();let a=attack(Phase::Recovery);s.parry(Some(a));
        let mut done=a;done.actionable=true;s.observe_opponent(done);
        assert_eq!(s.hit(a,true),Some(Rank::Great));
    }
    #[test] fn parry_credit_does_not_carry_to_a_new_move() {
        let mut s=State::new();let a=attack(Phase::Recovery);s.parry(Some(a));
        let mut next=a;next.epoch+=1;s.observe_opponent(next);
        assert_eq!(s.hit(next,true),Some(Rank::Great));
    }
    #[test] fn one_reward_per_opponent_attack_not_per_multihit() {
        let mut s=State::new();let a=attack(Phase::Recovery);
        assert_eq!(s.hit(a,true),Some(Rank::Great));assert_eq!(s.hit(a,true),None);
        let mut next=a;next.epoch+=1;assert_eq!(s.hit(next,true),Some(Rank::Great));
    }
    #[test] fn replacement_fighter_does_not_inherit_credit() {
        let mut s=State::new();let a=attack(Phase::Recovery);s.parry(Some(a));
        let mut next=a;next.object+=1;s.observe_opponent(next);
        assert_eq!(s.hit(next,true),Some(Rank::Great));
    }
}
