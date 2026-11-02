#![feature(proc_macro_hygiene)]
use smash::{app::{self, lua_bind::*, sv_battle_object, utility, BattleObjectModuleAccessor},
    lib::lua_const::*, lua2cpp::L2CFighterCommon, phx::{Hash40, Vector3f}};
use smashline::{Agent, Main};
use skyline::hooks::{getRegionAddress,Region};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
use std::io::Write;
static LOG_PATH: Mutex<Option<std::path::PathBuf>> = Mutex::new(None);
static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
static CALLBACK_SEEN: AtomicBool = AtomicBool::new(false);
static PANCAKE_CALLBACK_SEEN: AtomicBool = AtomicBool::new(false);
fn trace(message: &str) {
    println!("[kirby_quest] {}",message);
    if LOG_COUNT.fetch_add(1,Ordering::Relaxed)>=96 {return;}
    if let Some(path)=LOG_PATH.lock().unwrap().as_ref() {
        if let Ok(mut file)=std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _=writeln!(file,"{}",message);
        }
    }
}
mod rules;
mod voice_feedback;
mod profiles;
mod special_profiles;
mod damage_state;
mod damage_layout;
mod damage_feedback;
mod evasion_rules;
mod evasion_profiles;
mod evasion;
mod pancake_rules;
mod pancake_profiles;
mod pancake;
mod native_vector;
use rules::{Attack, Phase, Rank, State, AttackHistory};
static TECHS: Mutex<[rules::TechFeedback;8]> = Mutex::new([rules::TechFeedback::new();8]);
static SPITS:Mutex<rules::SpitRegistry>=Mutex::new(rules::SpitRegistry::new());
static HISTORY: Mutex<[AttackHistory;8]> = Mutex::new([AttackHistory::new();8]);
static GRABS: Mutex<[rules::CaptureSession;8]> = Mutex::new([rules::CaptureSession::new();8]);
#[derive(Clone,Copy)]
struct ArmorHit {attacker:usize,source:u32,target:u32,attack:Attack,age:u8}
static ARMOR_HITS: Mutex<[Option<ArmorHit>;8]> = Mutex::new([None;8]);

#[derive(Clone, Copy)]
struct Track { object: usize, motion: u64, status: i32, frame: f32, epoch: u64, parrying: bool }
impl Track { const fn new() -> Self { Self { object: 0, motion: 0, status: -1, frame: -1.0, epoch: 0, parrying: false } } }
static TRACK: Mutex<[Track; 8]> = Mutex::new([Track::new(); 8]);
static RULES: Mutex<[State; 8]> = Mutex::new([State::new(); 8]);
use voice_feedback::{Feedback, Decision};
static FEEDBACK: Mutex<[Feedback; 8]> = Mutex::new([Feedback::new(); 8]);

#[derive(serde::Deserialize, Clone)]
struct Settings {
    #[serde(default="default_pancake_run_stop")] pancake_run_stop_margin:f32,
    #[serde(default="default_pancake_margin")] pancake_horizontal_margin:f32,
    #[serde(default="default_dodge_near")] dodge_near_distance:f32,
    #[serde(default = "default_dedede_height")] dedede_height:f32,
    #[serde(default = "default_metaknight_height")] metaknight_height:f32,
    #[serde(default = "default_damage_height")] damage_height:f32,
    #[serde(default = "default_damage_scale")] damage_scale:f32,
    #[serde(default = "default_slow_start")] slow_startup_frames: f32,
    #[serde(default = "default_slow_end")] slow_recovery_frames: f32,
    #[serde(default = "default_assets_ready")] assets_ready: bool,
    #[serde(default = "default_height")] height: f32,
    #[serde(default = "default_scale")] scale: f32,
    #[serde(default = "default_voice_volume")] voice_volume: f32,
    #[serde(default = "default_quest_sound_volume")] quest_sound_volume: f32,
}
fn default_pancake_run_stop()->f32 {7.0}
fn default_pancake_margin()->f32 {5.0}
fn default_dodge_near()->f32 {20.0}
fn default_dedede_height()->f32 {26.0}
fn default_metaknight_height()->f32 {18.0}
fn default_damage_height()->f32 {9.0}
fn default_damage_scale()->f32 {1.0}
fn default_slow_start() -> f32 { 25.0 }
fn default_slow_end() -> f32 { 30.0 }
fn default_height() -> f32 { 16.0 }
fn default_scale() -> f32 { 1.0 }
fn default_assets_ready() -> bool { true }
fn default_voice_volume() -> f32 { 2.0 }
fn default_quest_sound_volume() -> f32 { 1.5 }
impl Settings { const fn new() -> Self { Self { pancake_run_stop_margin:7.0,pancake_horizontal_margin:5.0,dodge_near_distance:20.0,dedede_height:26.0,metaknight_height:18.0,damage_height:9.0,damage_scale:1.0,slow_startup_frames: 25.0, slow_recovery_frames: 30.0, assets_ready: true, height: 16.0, scale: 1.0, voice_volume: 2.0, quest_sound_volume: 1.5 } } }
static SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());

unsafe fn quest_fighter(boma:*mut BattleObjectModuleAccessor)->bool {
    !boma.is_null() && [*FIGHTER_KIND_KIRBY,*FIGHTER_KIND_DEDEDE,*FIGHTER_KIND_METAKNIGHT].contains(&utility::get_kind(&mut *boma))
}
unsafe fn entry(boma: *mut BattleObjectModuleAccessor) -> Option<usize> {
    if boma.is_null() || utility::get_category(&mut *boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER { return None; }
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if (0..8).contains(&id) { Some(id as usize) } else { None }
}
unsafe fn source_fighter(id: u32) -> Option<*mut BattleObjectModuleAccessor> {
    if id == u32::MAX || !sv_battle_object::is_active(id) { return None; }
    let spit_owner=SPITS.lock().unwrap().owner(id);
    if let Some(owner)=spit_owner {
        if owner!=id && sv_battle_object::is_active(owner) {
            let fighter=sv_battle_object::module_accessor(owner);
            if entry(fighter).is_some() && utility::get_kind(&mut *fighter)==*FIGHTER_KIND_DEDEDE {return Some(fighter);}
        }
    }
    let boma=sv_battle_object::module_accessor(id);
    if entry(boma).is_some() { return Some(boma); }
    if boma.is_null() || utility::get_category(&mut *boma) != *BATTLE_OBJECT_CATEGORY_WEAPON { return None; }
    let owner=WorkModule::get_int(boma,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if owner == id || owner == u32::MAX || !sv_battle_object::is_active(owner) { return None; }
    let owner=sv_battle_object::module_accessor(owner);
    entry(owner).map(|_|owner)
}
unsafe fn sample(boma: *mut BattleObjectModuleAccessor) -> Option<Attack> {
    let slot=entry(boma)?;
    let motion=MotionModule::motion_kind(boma);
    let status=StatusModule::status_kind(boma);
    let frame=MotionModule::frame(boma);
    let epoch={
        let mut tracks=TRACK.lock().unwrap();let t=&mut tracks[slot];
        if t.object != boma as usize || t.motion != motion || t.status != status || frame+0.25 < t.frame {
            t.epoch=t.epoch.wrapping_add(1);
        }
        t.object=boma as usize;t.motion=motion;t.status=status;t.frame=frame;t.epoch
    };
    let kind=utility::get_kind(&mut *boma);
    // Native statuses identify the move family even when a fighter uses a
    // differently named motion (for example an unangled forward attack).
    let canonical=if status==*FIGHTER_STATUS_KIND_ATTACK_S4 {Some("attack_s4_s")}
        else if status==*FIGHTER_STATUS_KIND_ATTACK_HI4 {Some("attack_hi4")}
        else if status==*FIGHTER_STATUS_KIND_ATTACK_LW4 {Some("attack_lw4")}
        else if status==*FIGHTER_STATUS_KIND_ATTACK_S3 {Some("attack_s3_s")}
        else if status==*FIGHTER_STATUS_KIND_ATTACK_HI3 {Some("attack_hi3")}
        else if status==*FIGHTER_STATUS_KIND_ATTACK_LW3 {Some("attack_lw3")}
        else if status==*FIGHTER_STATUS_KIND_ATTACK_DASH {Some("attack_dash")}
        else {None};
    let special=special_profiles::find(kind,motion);
    let mode=special.as_ref().map(|p|p.mode).unwrap_or(0);
    let profile=special.map(|p|profiles::Profile{first:p.first,last:p.last,smash:p.smash}).or_else(||profiles::find(kind,motion)).or_else(||canonical.and_then(|name|profiles::find(kind,Hash40::new(name).hash)));
    let native_smash=[*FIGHTER_STATUS_KIND_ATTACK_S4,*FIGHTER_STATUS_KIND_ATTACK_HI4,*FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status);
    let cancel=FighterMotionModuleImpl::get_cancel_frame(boma,Hash40::new_raw(motion),false);
    let hold=[*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status);
    let end=MotionModule::end_frame(boma);
    let actionable=!hold && rules::actionable(frame,cancel,end,CancelModule::is_enable_cancel(boma));
    let active=(0..8).any(|id| AttackModule::is_attack(boma,id,false));
    let (phase,smash,slow)=if hold { (Phase::Startup,true,false) } else if let Some(p)=profile {
        let phase=if actionable {Phase::Unknown} else if active {Phase::Active} else if mode==1 {Phase::Startup} else if mode==2 {Phase::Recovery} else {rules::phase(frame,p.first,p.last,false,false)};
        let s=SETTINGS.lock().unwrap();
        (phase,p.smash||native_smash,p.first>=s.slow_startup_frames || (if cancel>0.0 {cancel} else {end})-p.last>=s.slow_recovery_frames)
    } else { (Phase::Unknown,false,false) };
    let armored=special_profiles::armored(kind,motion,frame) || (special_profiles::find(kind,motion).is_some() && DamageModule::is_no_reaction_mode_perfect(boma));
    Some(Attack { kind, motion, status, frame, opponent: slot, object: boma as usize, epoch, phase, smash, slow, actionable, armored })
}
fn rank_label(rank: Rank) -> &'static str {
    match rank { Rank::Good=>"good", Rank::Great=>"great", Rank::Excellent=>"awesome" }
}
unsafe fn reward_prefix(boma: *mut BattleObjectModuleAccessor) -> &'static str {
    if utility::get_kind(&mut *boma)==*FIGHTER_KIND_KIRBY {"se_kirby_quest"} else {"se_quest"}
}
unsafe fn request_fighter_effect(b:*mut BattleObjectModuleAccessor, name:&str, pos:&Vector3f, rot:&Vector3f, scale:f32)->u64 {
    EffectModule::req(b,Hash40::new(name),pos,rot,scale,0,-1,false,0)
}
unsafe fn fighter_effect_name(boma:*mut BattleObjectModuleAccessor,name:&str)->String {
    let kind=utility::get_kind(&mut *boma);
    let prefix=if kind==*FIGHTER_KIND_KIRBY {"kirby"} else if kind==*FIGHTER_KIND_DEDEDE {"dedede"} else {"metaknight"};
    format!("{}_{}",prefix,name)
}
unsafe fn quest_cue(boma: *mut BattleObjectModuleAccessor, rank: Rank) {
    let prefix=reward_prefix(boma);
    let cue=SoundModule::play_se_no3d(boma,Hash40::new(&format!("{}_cue_{}",prefix,rank_label(rank))),true,false);
    let gain=SETTINGS.lock().unwrap().quest_sound_volume;
    SoundModule::set_se_vol(boma,cue as i32,if gain.is_finite(){gain.clamp(0.0,4.0)}else{1.5},0);
}
unsafe fn voice(boma: *mut BattleObjectModuleAccessor, rank: Rank) {
    let prefix=reward_prefix(boma);
    let sound=SoundModule::play_se_no3d(boma,Hash40::new(&format!("{}_{}",prefix,rank_label(rank))),true,false);
    let gain=SETTINGS.lock().unwrap().voice_volume;
    trace(&format!("sound={} handle={} gain={}",rank_label(rank),sound,gain));
    SoundModule::set_se_vol(boma,sound as i32,if gain.is_finite(){gain.clamp(0.0,4.0)}else{2.0},0);
}
unsafe fn announce(boma: *mut BattleObjectModuleAccessor, rank: Rank, cause: &str) {
    let label=rank_label(rank);
    trace(&format!("rating={} cause={}",label,cause));
    let s=SETTINGS.lock().unwrap().clone();
    if !s.assets_ready { return; }
    // Restart the card for the new accomplishment instead of overlapping two
    // identical planes when a parry is followed immediately by a punish.
    for name in ["quest_good","quest_great","quest_awesome"] {
        EffectModule::kill_kind(boma,Hash40::new(&fighter_effect_name(boma,name)),false,true);
    }
    let mut pos=*PostureModule::pos(boma);pos.y+=match utility::get_kind(&mut *boma) {k if k==*FIGHTER_KIND_DEDEDE=>s.dedede_height,k if k==*FIGHTER_KIND_METAKNIGHT=>s.metaknight_height,_=>s.height};
    let rot=Vector3f{x:0.0,y:0.0,z:0.0};
    let effect=request_fighter_effect(boma,&fighter_effect_name(boma,&format!("quest_{}",label)),&pos,&rot,s.scale);
    if effect != 0 && EffectModule::is_exist_effect(boma,effect as u32) {
        EffectModule::set_billboard(boma,effect as u32,true);
    } else { trace(&format!("rating effect unavailable {}",fighter_effect_name(boma,&format!("quest_{}",label)))); }
    trace(&format!("graphic={} handle={}",label,effect));
    quest_cue(boma,rank);
    if let Some(slot)=entry(boma) {
        let decision={
            let mut feedback=FEEDBACK.lock().unwrap();let f=&mut feedback[slot];
            let playing=f.active.map(|prior|SoundModule::is_playing(boma,Hash40::new(&format!("{}_{}",reward_prefix(boma),rank_label(prior))))).unwrap_or(false);
            f.request(rank,playing)
        };
        match decision {
            Decision::Skip=>{},
            Decision::Play=>voice(boma,rank),
            Decision::Interrupt(prior)=>{
                SoundModule::stop_se(boma,Hash40::new(&format!("{}_{}",reward_prefix(boma),rank_label(prior))),0);
                voice(boma,rank);
            }
        }
    }
}
unsafe extern "C" fn frame(agent: &mut L2CFighterCommon) {
    let boma=agent.module_accessor;let slot=match entry(boma) {Some(x)=>x,None=>return};
    if !CALLBACK_SEEN.swap(true,Ordering::Relaxed) {trace("fighter callback active");}
    let status=StatusModule::status_kind(boma);
    let ready=app::sv_information::is_ready_go();let training=app::smashball::is_training_mode();
    let stock_reset=[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH].contains(&status);
    let reset=pancake_rules::gameplay_reset(ready,training,stock_reset);
    if utility::get_kind(&mut *boma)==*FIGHTER_KIND_KIRBY && !PANCAKE_CALLBACK_SEEN.swap(true,Ordering::Relaxed) {trace(&format!("Kirby callback: ready={} training={} reset={} status={} situation={}",ready,training,reset,status,StatusModule::situation_kind(boma)));}
    damage_feedback::observe(boma,slot,reset);
    if reset {
        if let Some(rank)=FEEDBACK.lock().unwrap()[slot].active {
            SoundModule::stop_se(boma,Hash40::new(&format!("{}_{}",reward_prefix(boma),rank_label(rank))),0);
        }
        evasion::reset(slot);
        pancake::reset(slot);
        SPITS.lock().unwrap().reset((*boma).battle_object_id);
        damage_feedback::flush_moves(boma,slot,true);
        TECHS.lock().unwrap()[slot]=rules::TechFeedback::new();
        TRACK.lock().unwrap()[slot]=Track::new();RULES.lock().unwrap()[slot]=State::new();FEEDBACK.lock().unwrap()[slot]=Feedback::new();HISTORY.lock().unwrap()[slot]=AttackHistory::new();GRABS.lock().unwrap()[slot]=rules::CaptureSession::new();ARMOR_HITS.lock().unwrap()[slot]=None;return;
    }
    let observation=sample(boma);
    damage_feedback::flush_moves(boma,slot,false);
    let captured=CaptureModule::is_capture(boma);
    let reaction=damage_reaction(boma)||captured;
    // Capture ownership covers normal grabs and Kirby's inhale, without
    // changing their gameplay scripts or counting pummels as new grabs.
    if captured {
        let owner=CaptureModule::catch_object_id(boma) as u32;
        if let Some(kirby)=source_fighter(owner) {
            if quest_fighter(kirby) {
                if let Some(k)=entry(kirby) {
                    let target=(*boma).battle_object_id;
                    let owner_status=StatusModule::status_kind(kirby);
                    let inhale=inhale_status(utility::get_kind(&mut *kirby),owner_status);
                    let grab=[*FIGHTER_STATUS_KIND_CATCH,*FIGHTER_STATUS_KIND_CATCH_DASH,*FIGHTER_STATUS_KIND_CATCH_TURN,*FIGHTER_STATUS_KIND_CATCH_PULL,*FIGHTER_STATUS_KIND_CATCH_WAIT,*FIGHTER_STATUS_KIND_CATCH_ATTACK,*FIGHTER_STATUS_KIND_CATCH_DASH_PULL,*FIGHTER_STATUS_KIND_THROW].contains(&owner_status);
                    let fresh=(inhale||grab) && GRABS.lock().unwrap()[k].begin(target,inhale);
                    if fresh {let prior=HISTORY.lock().unwrap()[slot].before_damage(boma as usize);let rank=RULES.lock().unwrap()[k].grab(prior);announce(kirby,rank,"grab/inhale");}
                }
            }
        }
    }
    // Confirm real damage reaction, not armor hitstop, before celebrating
    // a hit that penetrated startup armor.
    let pending=ARMOR_HITS.lock().unwrap()[slot].take();
    if let Some(mut hit)=pending {
        if hit.target==(*boma).battle_object_id && hit.attack.object==boma as usize {
            let interrupted=damage_status(status) || damage_status(StatusModule::status_kind_next(boma));
            if interrupted {
                let rank=RULES.lock().unwrap()[hit.attacker].armor_break(hit.attack,true);
                if let (Some(rank),Some(kirby))=(rank,source_fighter(hit.source)) {announce(kirby,rank,"startup armor broken");}
            } else {
                hit.age=hit.age.saturating_add(1);
                if hit.age<30 && (hit.age<2 || StopModule::is_stop(boma)) {ARMOR_HITS.lock().unwrap()[slot]=Some(hit);}
            }
        }
    }
    if let Some(attack)=observation {HISTORY.lock().unwrap()[slot].observe(attack,reaction);}
    if !reaction { if let Some(a)=observation { for s in RULES.lock().unwrap().iter_mut() {s.observe_opponent(a);} } }
    if !quest_fighter(boma) {return;}
    evasion::frame(boma,slot);
    pancake::frame(boma,slot);
    let tech=if [*FIGHTER_STATUS_KIND_PASSIVE,*FIGHTER_STATUS_KIND_PASSIVE_FB].contains(&status) {Some(Rank::Great)}
        else if [*FIGHTER_STATUS_KIND_PASSIVE_WALL,*FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP,*FIGHTER_STATUS_KIND_PASSIVE_CEIL].contains(&status) {Some(Rank::Excellent)} else {None};
    let reward=TECHS.lock().unwrap()[slot].observe(tech);
    if let Some(rank)=reward {announce(boma,rank,"successful tech");}
    if utility::get_kind(&mut *boma)==*FIGHTER_KIND_DEDEDE && status==*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT {
        let shot=WorkModule::get_int(boma,*FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
        if sv_battle_object::is_active(shot) {let weapon=sv_battle_object::module_accessor(shot);if !weapon.is_null() && utility::get_category(&mut *weapon)==*BATTLE_OBJECT_CATEGORY_WEAPON {SPITS.lock().unwrap().record((*boma).battle_object_id,shot);}}
    }
    let held=GRABS.lock().unwrap()[slot].target;
    if let Some(target)=held {
        let valid=if sv_battle_object::is_active(target) {let victim=sv_battle_object::module_accessor(target);!victim.is_null() && CaptureModule::is_capture(victim) && CaptureModule::catch_object_id(victim) as u32==(*boma).battle_object_id} else {false};
        GRABS.lock().unwrap()[slot].observe(inhale_status(utility::get_kind(&mut *boma),status),valid);
    }
    FEEDBACK.lock().unwrap()[slot].tick();
    let parrying=status == *FIGHTER_STATUS_KIND_GUARD_DAMAGE
        && WorkModule::is_flag(boma,*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD);
    let fresh={let mut t=TRACK.lock().unwrap();let fresh=parrying&&!t[slot].parrying;t[slot].parrying=parrying;fresh};
    if fresh {
        let source=WorkModule::get_int(boma,*FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID) as u32;
        let attack=source_fighter(source).and_then(|x|sample(x));
        let rank=RULES.lock().unwrap()[slot].parry(attack);
        announce(boma,rank,"parry");
    }
}
unsafe fn damage_reaction(boma: *mut BattleObjectModuleAccessor) -> bool {
    let status=StatusModule::status_kind(boma);
    FighterStopModuleImpl::is_damage_stop(boma) || [*FIGHTER_STATUS_KIND_DAMAGE,*FIGHTER_STATUS_KIND_DAMAGE_AIR,*FIGHTER_STATUS_KIND_DAMAGE_FLY,*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,*FIGHTER_STATUS_KIND_DAMAGE_FALL,*FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status)
}
unsafe fn inhale_status(kind:i32,status:i32)->bool {
    if kind==*FIGHTER_KIND_KIRBY {status==*FIGHTER_STATUS_KIND_SPECIAL_N || (status>=*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP && status<=*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT)}
    else if kind==*FIGHTER_KIND_DEDEDE {status==*FIGHTER_STATUS_KIND_SPECIAL_N || (status>=*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_LOOP && status<=*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT)} else {false}
}
unsafe fn damage_status(status:i32)->bool {
    [*FIGHTER_STATUS_KIND_DAMAGE,*FIGHTER_STATUS_KIND_DAMAGE_AIR,*FIGHTER_STATUS_KIND_DAMAGE_FLY,*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,*FIGHTER_STATUS_KIND_DAMAGE_FALL,*FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status)
}
static mut COLLISION_OFFSET: usize=0;
#[skyline::hook(offset = COLLISION_OFFSET)]
unsafe fn collision_hit(manager: *mut app::FighterManager, attacker_id: u32, defender_id: u32, damage: f32, attack_kind: i32, meteor: bool) -> u64 {
    // Obtain identities from the event, not delayed SUCCEED_ATTACKER flags.
    let source=source_fighter(attacker_id);
    let target=if sv_battle_object::is_active(defender_id) {sv_battle_object::module_accessor(defender_id)} else {std::ptr::null_mut()};
    // The native star's IS_WEAPON flag distinguishes an inhaled weapon from
    // a star made by spitting out a captured fighter or discarding a Copy Ability.
    let star=if damage.is_finite() && damage>0.0 {
        source.and_then(|kirby| {
            let slot=entry(kirby)?;let opponent=entry(target)?;
            if slot==opponent || !quest_fighter(kirby) {return None;}
            let weapon=sv_battle_object::module_accessor(attacker_id);
            if weapon.is_null() || utility::get_category(&mut *weapon)!=*BATTLE_OBJECT_CATEGORY_WEAPON {return None;}
            let converted_star=utility::get_kind(&mut *kirby)==*FIGHTER_KIND_KIRBY
                && utility::get_kind(&mut *weapon)==*WEAPON_KIND_KIRBY_STARMISSILE
                && WorkModule::is_flag(weapon,*WEAPON_KIRBY_STARMISSILE_INSTANCE_WORK_ID_FLAG_IS_WEAPON);
            let dedede_spit=SPITS.lock().unwrap().owner(attacker_id)==Some((*kirby).battle_object_id);
            if !converted_star && !dedede_spit {return None;}
            let status=StatusModule::status_kind(target);
            if HitModule::get_whole(target,*HIT_STATUS_INVINCIBLE)!=0
                || [*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_ON,*FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&status) {return None;}
            Some((kirby,slot))
        })
    } else {None};
    let candidate=if damage.is_finite() && damage>0.0 {
        source.and_then(|kirby| {
            let attacker=entry(kirby)?;let opponent=entry(target)?;
            if attacker==opponent || !quest_fighter(kirby) {return None;}
            let status=StatusModule::status_kind(target);
            trace(&format!("Kirby collision received: damage={:.3} target_kind={} current_status={} current_frame={:.2}",damage,utility::get_kind(&mut *target),status,MotionModule::frame(target)));
            if HitModule::get_whole(target,*HIT_STATUS_INVINCIBLE)!=0 {return None;}
            if [*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_ON,*FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&status) {return None;}
            let current=sample(target)?;
            let cached=current.phase==Phase::Unknown && damage_reaction(target);
            let attack=if cached {HISTORY.lock().unwrap()[opponent].before_damage(target as usize)?} else {current};
            Some((kirby,attacker,attack,cached))
        })
    } else {None};
    damage_feedback::collision_begin(source,target,damage);
    let result=original!()(manager,attacker_id,defender_id,damage,attack_kind,meteor);
    damage_feedback::collision_end(target);
    if let Some((kirby,slot))=star {
        let rank=RULES.lock().unwrap()[slot].projectile_star(attacker_id,true,true);
        if let Some(rank)=rank {announce(kirby,rank,"inhaled projectile star hit");}
        return result;
    }
    if let Some((kirby,attacker,attack,cached))=candidate {
        trace(&format!("collision: kind={} motion={:010x} status={} frame={:.2} phase={:?} smash={} slow={} actionable={} cached={}",attack.kind,attack.motion,attack.status,attack.frame,attack.phase,attack.smash,attack.slow,attack.actionable,cached));
        if attack.armored && attack.phase==Phase::Startup {
            ARMOR_HITS.lock().unwrap()[attack.opponent]=Some(ArmorHit{attacker,source:(*kirby).battle_object_id,target:defender_id,attack,age:0});
            return result;
        }
        let rank=RULES.lock().unwrap()[attacker].hit(attack,true);
        if let Some(rank)=rank {announce(kirby,rank,if attack.phase==Phase::Startup{"counter"}else{"punish"});}
    }
    result
}
#[skyline::main(name = "kirby_quest_effects")]
pub fn main() {
    // Keep settings usable when the mod folder is renamed.
    if let Ok(folders)=std::fs::read_dir("sd:/ultimate/mods") {
        for folder in folders.flatten() {
            let file=folder.path().join("kirby_quest.toml");
            if let Ok(text)=std::fs::read_to_string(file) {
                if let Ok(settings)=toml::from_str::<Settings>(&text) {
                    *SETTINGS.lock().unwrap()=settings;
                    let path=folder.path().join("kirby_quest_runtime.log");
                    let _=std::fs::write(&path,concat!("Kirby Quest Effects ",env!("CARGO_PKG_VERSION")," loaded\n"));
                    *LOG_PATH.lock().unwrap()=Some(path);break;
                }
            }
        }
    }
    unsafe {
        let start=getRegionAddress(Region::Text) as *const u8;
        let size=getRegionAddress(Region::Rodata) as usize-start as usize;
        let text=std::slice::from_raw_parts(start,size);
        let mut display=skyline::nn::oe::DisplayVersion{name:[0;16]};
        skyline::nn::oe::GetDisplayVersion(&mut display);
        let count=display.name.iter().position(|b|*b==0).unwrap_or(display.name.len());
        let version=String::from_utf8_lossy(&display.name[..count]);
        trace(&format!("game version={}",version));
        damage_feedback::install(text,&version);
        if text.len()>0x67a7b0+16 {trace(&format!("working-mod hook entry={:02x?}",&text[0x67a7b0..0x67a7b0+16]));}
        if let Some(offset)=rules::collision_offset_for_version(text,&version) {
            COLLISION_OFFSET=offset;skyline::install_hook!(collision_hit);
            trace(&format!("native collision hook installed at {:x}",offset));
        } else {trace("ERROR: native collision hook not found uniquely; hit ratings unavailable");}
    }
    Agent::new("fighter").on_line(Main,frame).install();
    Agent::new("weapon").on_line(Main,pancake::weapon_frame).install();
}

