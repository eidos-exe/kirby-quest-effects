use super::*;
use super::evasion_rules::{Session, Dodge, Threat, waiting_on_dodge};
#[derive(Clone, Copy)]
struct Tracker {
    session: Session,
    last_status: i32,
    frame: f32,
    origin: [f32; 3],
    clear_frames: u8,
    reported: bool,
    intangible_seen: bool,
}
impl Tracker {
    const fn new() -> Self {
        Self {
            session: Session::new(),
            last_status: -1,
            frame: -1.,
            origin: [0.; 3],
            clear_frames: 0,
            reported: false,
            intangible_seen: false,
        }
    }
}
static DODGES: Mutex<[Tracker; 8]> = Mutex::new([Tracker::new(); 8]);
fn xyz(v: Vector3f) -> [f32; 3] {
    [v.x, v.y, v.z]
}
unsafe fn active_attack(attacker: *mut BattleObjectModuleAccessor) -> bool {
    (0..8).any(|id| {
        if !AttackModule::is_attack(attacker, id, false) {
            return false;
        }
        let data = AttackModule::attack_data(attacker, id, false);
        !data.is_null() && (*data).power.is_finite() && (*data).power > 0.0
    })
}
unsafe fn threat_key(attack: Attack, profile: &evasion_profiles::Profile) -> Threat {
    Threat {
        opponent: attack.opponent,
        object: attack.object,
        group: if profile.rapid {
            Hash40::new("attack_100").hash
        } else {
            attack.motion
        },
        epoch: if profile.rapid { 0 } else { attack.epoch },
    }
}
unsafe fn resolved(threat: Threat) -> bool {
    let id = damage_state::OBJECT_IDS.lock().unwrap()[threat.opponent].unwrap_or(u32::MAX);
    if !sv_battle_object::is_active(id) {
        return true;
    }
    let opponent = sv_battle_object::module_accessor(id);
    if opponent as usize != threat.object {
        return true;
    }
    let attack = match sample(opponent) {
        Some(a) => a,
        None => return true,
    };
    let profile = match evasion_profiles::find(attack.kind, attack.motion) {
        Some(p) => p,
        None => return true,
    };
    if threat_key(attack, &profile) != threat {
        return true;
    }
    attack.actionable
        || (attack.frame > profile.last
            && !(0..8).any(|id| AttackModule::is_attack(opponent, id, false)))
}
pub unsafe fn reset(slot: usize) {
    DODGES.lock().unwrap()[slot] = Tracker::new();
}
pub unsafe fn frame(boma: *mut BattleObjectModuleAccessor, slot: usize) {
    let status = StatusModule::status_kind(boma);
    let motion_frame = MotionModule::frame(boma);
    let mode = if status == *FIGHTER_STATUS_KIND_ESCAPE {
        Some(Dodge::Spot)
    } else if status == *FIGHTER_STATUS_KIND_ESCAPE_F || status == *FIGHTER_STATUS_KIND_ESCAPE_B {
        Some(Dodge::Roll)
    } else {
        None
    };
    let damage = DamageModule::damage(boma, 0);
    let mut tracker = DODGES.lock().unwrap()[slot];
    if let Some(mode) = mode {
        if status != tracker.last_status || motion_frame + 0.25 < tracker.frame {
            tracker.session.begin(mode, damage);
            tracker.clear_frames = 0;
            tracker.reported = false;
            tracker.intangible_seen = false;
            trace(&format!("dodge begin: slot={} mode={:?}", slot, mode));
            tracker.origin = xyz(*PostureModule::pos(boma));
        }
    }
    let was_dodging = [
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
    ]
    .contains(&tracker.last_status);
    tracker.last_status = status;
    tracker.frame = motion_frame;
    let intangible = HitModule::get_status(boma, 0, 0) == *HIT_STATUS_XLU
        || HitModule::get_whole(boma, 0) == *HIT_STATUS_XLU;
    if mode.is_some() && intangible {
        tracker.intangible_seen = true;
        let near = SETTINGS.lock().unwrap().dodge_near_distance;
        let ids = *damage_state::OBJECT_IDS.lock().unwrap();
        for other in 0..8 {
            if other == slot {
                continue;
            }
            let id = ids[other].unwrap_or(u32::MAX);
            if !sv_battle_object::is_active(id) {
                continue;
            }
            let opponent = sv_battle_object::module_accessor(id);
            if entry(opponent) != Some(other)
                || TeamModule::team_no(opponent) == TeamModule::team_no(boma)
            {
                continue;
            }
            let pos = *PostureModule::pos(opponent);
            if (pos.x - tracker.origin[0]).abs() > near || (pos.y - tracker.origin[1]).abs() > near
            {
                continue;
            }
            let attack = match sample(opponent) {
                Some(a) => a,
                None => continue,
            };
            let profile = match evasion_profiles::find(attack.kind, attack.motion) {
                Some(p) => p,
                None => continue,
            };
            let overlap = active_attack(opponent);
            if overlap && !tracker.reported {
                trace(&format!(
                    "dodge threat: slot={} opponent={} multi={} frame={:.2}",
                    slot, other, profile.multi, attack.frame
                ));
                tracker.reported = true;
            }
            tracker
                .session
                .threat(threat_key(attack, &profile), profile.multi, true, overlap);
        }
    }
    let mut complete = tracker
        .session
        .pending
        .map(|p| resolved(p.threat))
        .unwrap_or(false);
    // A held rapid attack need not stop before a roll can count as an escape.
    // Require three actual active frames outside its reach after the roll ends;
    // empty frames between hits cannot advance this clearance check.
    if !complete && (mode.is_none() || (!intangible && tracker.intangible_seen)) {
        if let Some(candidate) = tracker.session.pending {
            if candidate.rank == Rank::Excellent {
                let id = damage_state::OBJECT_IDS.lock().unwrap()[candidate.threat.opponent]
                    .unwrap_or(u32::MAX);
                if sv_battle_object::is_active(id) {
                    let opponent = sv_battle_object::module_accessor(id);
                    if opponent as usize == candidate.threat.object {
                        if let Some(a) = sample(opponent) {
                            if let Some(p) = evasion_profiles::find(a.kind, a.motion) {
                                if active_attack(opponent) && p.rapid {
                                    let current = *PostureModule::pos(boma);
                                    let enemy = *PostureModule::pos(opponent);
                                    let dx = current.x - enemy.x;
                                    let near = SETTINGS.lock().unwrap().dodge_near_distance;
                                    let clear =
                                        dx.abs() > near || dx * PostureModule::lr(opponent) < -6.0;
                                    tracker.clear_frames = if clear {
                                        tracker.clear_frames.saturating_add(1)
                                    } else {
                                        0
                                    };
                                    complete = tracker.clear_frames >= 3;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Use current reaction state and received damage. Pending next-status and
    // capture-module flags can outlive the interaction that originally set them.
    let captured = status >= *FIGHTER_STATUS_KIND_CAPTURE_PULLED
        && status <= *FIGHTER_STATUS_KIND_CAPTURE_JUMP;
    let failed = damage_status(status) || captured;
    let protected = waiting_on_dodge(mode.is_some(), tracker.intangible_seen, intangible);
    let pending = tracker.session.pending.is_some();
    let rank = tracker.session.finish(protected, damage, failed, complete);
    if was_dodging && mode.is_none() {
        trace(&format!(
            "dodge end: slot={} intangible={} threat={} pending={} failed={} resolved={}",
            slot, tracker.intangible_seen, tracker.reported, pending, failed, complete
        ));
    }
    DODGES.lock().unwrap()[slot] = tracker;
    if let Some(rank) = rank {
        announce(boma, rank, "nearby active-attack dodge escape");
    }
}
