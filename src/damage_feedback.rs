use super::*;
use super::{damage_state, damage_layout};
static NUMBER_HANDLES: Mutex<[[[u32; 12]; 8]; 8]> = Mutex::new([[[0; 12]; 8]; 8]);
unsafe fn quest_source(slot: usize) -> Option<*mut BattleObjectModuleAccessor> {
    if slot >= 8 {
        return None;
    }
    let id = damage_state::OBJECT_IDS.lock().unwrap()[slot]?;
    source_fighter(id).filter(|b| quest_fighter(*b))
}
unsafe fn emit_numbers(
    source: *mut BattleObjectModuleAccessor,
    target: *mut BattleObjectModuleAccessor,
    damage: f32,
) -> [u32; 12] {
    let text = match damage_state::damage_text(damage) {
        Some(x) => x,
        None => return [0; 12],
    };
    let mut handles = [0; 12];
    trace(&format!(
        "damage number={} source_slot={:?} target_slot={:?}",
        text,
        entry(source),
        entry(target)
    ));
    let pos = *PostureModule::pos(target);
    let settings = SETTINGS.lock().unwrap().clone();
    for (i, (glyph, x)) in damage_layout::positions(&text).into_iter().enumerate() {
        let name = if glyph == '.' {
            "quest_num_decimal".to_string()
        } else {
            format!("quest_num_{}", glyph)
        };
        let p = Vector3f {
            x: pos.x + x * settings.damage_scale,
            y: pos.y + settings.damage_height,
            z: pos.z,
        };
        let rot = Vector3f {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let h = request_fighter_effect(
            source,
            &fighter_effect_name(source, &name),
            &p,
            &rot,
            settings.damage_scale,
        );
        if h != 0 && EffectModule::is_exist_effect(source, h as u32) {
            EffectModule::set_billboard(source, h as u32, true);
        }
        if i < 12 {
            handles[i] = h as u32;
        }
        trace(&format!("damage glyph={} handle={}", glyph, h));
    }
    handles
}
unsafe fn multihit_motion(source: *mut BattleObjectModuleAccessor) -> Option<(u64, f32, f32)> {
    let a = sample(source)?;
    let jab = [
        "attack_11",
        "attack_12",
        "attack_13",
        "attack_100_start",
        "attack_100",
        "attack_100_end",
    ]
    .iter()
    .any(|n| a.motion == Hash40::new(n).hash);
    let sequence = damage_state::JABS.lock().unwrap()[a.opponent].observe(jab, a.epoch);
    if let Some(token) = sequence {
        return Some((token, 1000000.0, 1000000.0));
    }
    let kind = utility::get_kind(&mut *source);
    let (finish, end) = if kind == *FIGHTER_KIND_KIRBY {
        if a.motion == Hash40::new("attack_air_f").hash {
            (25.0, 28.0)
        } else if a.motion == Hash40::new("attack_air_lw").hash {
            (34.0, 35.0)
        } else {
            return None;
        }
    } else if kind == *FIGHTER_KIND_METAKNIGHT {
        if a.motion == Hash40::new("attack_air_f").hash {
            (15.0, 16.0)
        } else if a.motion == Hash40::new("attack_air_b").hash {
            (20.0, 22.0)
        } else {
            return None;
        }
    } else if kind == *FIGHTER_KIND_DEDEDE && a.motion == Hash40::new("attack_air_hi").hash {
        (24.0, 26.0)
    } else {
        return None;
    };
    Some((a.epoch, finish, end))
}
unsafe fn numbers(
    source: *mut BattleObjectModuleAccessor,
    target: *mut BattleObjectModuleAccessor,
    damage: f32,
) {
    let (attacker, victim) = match (entry(source), entry(target)) {
        (Some(a), Some(b)) => (a, b),
        _ => return,
    };
    let motion = multihit_motion(source);
    let key = motion.map(|(epoch, _, _)| (epoch, source as usize, target as usize));
    let old_key = damage_state::MULTI.lock().unwrap()[attacker][victim].key;
    damage_state::MULTI.lock().unwrap()[attacker][victim].flush_unless(key);
    if old_key != key {
        NUMBER_HANDLES.lock().unwrap()[attacker][victim] = [0; 12];
    }
    if let (Some(key), Some((_, finish, _))) = (key, motion) {
        let total = {
            let mut all = damage_state::MULTI.lock().unwrap();
            let m = &mut all[attacker][victim];
            m.add(key, damage, MotionModule::frame(source) >= finish - 0.25)
                .unwrap_or_else(|| m.total())
        };
        let old = NUMBER_HANDLES.lock().unwrap()[attacker][victim];
        for h in old {
            if h != 0 && EffectModule::is_exist_effect(source, h) {
                EffectModule::kill(source, h, false, true);
            }
        }
        let handles = emit_numbers(source, target, total);
        NUMBER_HANDLES.lock().unwrap()[attacker][victim] = handles;
    } else {
        emit_numbers(source, target, damage);
    }
}

pub unsafe fn flush_moves(source: *mut BattleObjectModuleAccessor, slot: usize, reset: bool) {
    if reset {
        damage_state::JABS.lock().unwrap()[slot] = damage_state::JabSequence::new();
        damage_state::MULTI.lock().unwrap()[slot] = [damage_state::MultiHit::new(); 8];
        NUMBER_HANDLES.lock().unwrap()[slot] = [[0; 12]; 8];
        return;
    }
    if !quest_fighter(source) {
        return;
    }
    let current = multihit_motion(source).filter(|(_, _, end)| MotionModule::frame(source) < *end);
    for victim in 0..8 {
        let id = damage_state::OBJECT_IDS.lock().unwrap()[victim].unwrap_or(u32::MAX);
        let target = if sv_battle_object::is_active(id) {
            sv_battle_object::module_accessor(id)
        } else {
            std::ptr::null_mut()
        };
        let key = current
            .filter(|_| !target.is_null())
            .map(|(epoch, _, _)| (epoch, source as usize, target as usize));
        let old_key = damage_state::MULTI.lock().unwrap()[slot][victim].key;
        damage_state::MULTI.lock().unwrap()[slot][victim].flush_unless(key);
        if old_key != key {
            NUMBER_HANDLES.lock().unwrap()[slot][victim] = [0; 12];
        }
    }
}
unsafe fn native_attacker(boma: *mut BattleObjectModuleAccessor) -> Option<usize> {
    let mask = WorkModule::get_int(
        boma,
        *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID,
    ) as u16;
    damage_state::single_attacker(mask)
}
unsafe fn show_damage(target: *mut BattleObjectModuleAccessor, damage: f32) {
    let victim = match entry(target) {
        Some(x) => x,
        None => return,
    };
    let event = damage_state::OWNERS.lock().unwrap()[victim];
    if let Some(owner) = event.filter(|o| o.target == target as usize && o.age <= 6) {
        if let Some(source) = source_fighter(owner.source) {
            if source != target && quest_fighter(source) {
                numbers(source, target, damage);
            }
        }
        return;
    }
    if CaptureModule::is_capture(target) {
        if let Some(source) = source_fighter(CaptureModule::catch_object_id(target) as u32) {
            if source != target && quest_fighter(source) {
                numbers(source, target, damage);
                return;
            }
        }
    }
    if let Some(attacker) = native_attacker(target) {
        if let Some(source) = quest_source(attacker) {
            if source != target {
                numbers(source, target, damage);
            }
        }
    }
}
// Record ownership from the native hit event before its damage callbacks run.
// Record other fighters too, so an opponent's later hit cannot inherit Kirby credit.
pub unsafe fn collision_begin(
    source: Option<*mut BattleObjectModuleAccessor>,
    target: *mut BattleObjectModuleAccessor,
    damage: f32,
) {
    let slot = match entry(target) {
        Some(x) => x,
        None => return,
    };
    if !damage.is_finite() || damage <= 0.0 {
        return;
    }
    let status = StatusModule::status_kind(target);
    if [
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
    ]
    .contains(&status)
    {
        return;
    }
    let pending = damage_state::DAMAGE.lock().unwrap()[slot].observe(
        target as usize,
        DamageModule::damage(target, 0),
        false,
    );
    if let Some(damage) = pending {
        show_damage(target, damage);
    }
    damage_state::OBJECT_IDS.lock().unwrap()[slot] = Some((*target).battle_object_id);
    damage_state::OWNERS.lock().unwrap()[slot] = Some(damage_state::DamageOwner {
        source: source.map(|b| (*b).battle_object_id).unwrap_or(u32::MAX),
        target: target as usize,
        age: 0,
    });
}
pub unsafe fn collision_end(target: *mut BattleObjectModuleAccessor) {
    if let Some(slot) = entry(target) {
        let pending = damage_state::DAMAGE.lock().unwrap()[slot].observe(
            target as usize,
            DamageModule::damage(target, 0),
            false,
        );
        if let Some(damage) = pending {
            show_damage(target, damage);
        }
    }
}

pub unsafe fn observe(boma: *mut BattleObjectModuleAccessor, slot: usize, reset: bool) {
    damage_state::OBJECT_IDS.lock().unwrap()[slot] = Some((*boma).battle_object_id);
    if reset {
        damage_state::OWNERS.lock().unwrap()[slot] = None;
    }

    if reset {
        for row in damage_state::MULTI.lock().unwrap().iter_mut() {
            row[slot] = damage_state::MultiHit::new();
        }
    }
    let pending = damage_state::DAMAGE.lock().unwrap()[slot].observe(
        boma as usize,
        DamageModule::damage(boma, 0),
        reset,
    );
    if let Some(damage) = pending {
        show_damage(boma, damage);
    }
    if !StopModule::is_stop(boma) {
        if let Some(owner) = damage_state::OWNERS.lock().unwrap()[slot].as_mut() {
            owner.age = owner.age.saturating_add(1);
        }
    }
}
static mut DAMAGE_OFFSET: usize = 0;
#[skyline::hook(offset = DAMAGE_OFFSET)]
unsafe fn damage_hook(object: *mut app::BattleObject, log: *const u8) {
    if object.is_null() {
        original!()(object, log);
        return;
    }
    let target = (*object).module_accessor;
    let slot = match entry(target) {
        Some(slot) => slot,
        None => {
            original!()(object, log);
            return;
        }
    };
    let before_percent = DamageModule::damage(target, 0);
    let before_total =
        WorkModule::get_float(target, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    // Flush any periodic damage that arrived before this direct hit.
    let pending =
        damage_state::DAMAGE.lock().unwrap()[slot].observe(target as usize, before_percent, false);
    if let Some(damage) = pending {
        show_damage(target, damage);
    }
    original!()(object, log);
    let after_percent = DamageModule::damage(target, 0);
    let after_total =
        WorkModule::get_float(target, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    damage_state::DAMAGE.lock().unwrap()[slot].observe(target as usize, after_percent, false);
    // Percent can update later than this callback. Under the percent cap,
    // wait for that real increase instead of counting a reported hit twice.
    let applied =
        damage_state::callback_damage(before_percent, after_percent, before_total, after_total);
    if let Some(damage) = applied {
        show_damage(target, damage);
    }
}

pub unsafe fn install(text: &[u8], version: &str) {
    let signature: &[u8] = &[
        0xff, 0xc3, 0x06, 0xd1, 0xed, 0x33, 0x12, 0x6d, 0xeb, 0x2b, 0x13, 0x6d, 0xe9, 0x23, 0x14,
        0x6d, 0xfc, 0x6f, 0x15, 0xa9, 0xfa, 0x67, 0x16, 0xa9, 0xf8, 0x5f, 0x17, 0xa9,
    ];
    let matches: Vec<_> = text
        .windows(signature.len())
        .enumerate()
        .filter(|(_, b)| *b == signature)
        .map(|(i, _)| i)
        .collect();
    let fixed = 0x6310c0;
    let offset = if matches.len() == 1 {
        Some(matches[0])
    } else if matches!(version, "13.0.3" | "13.0.4" | "13.0.5")
        && text.len() > fixed + 28
        && text[fixed + 20..fixed + 28] == signature[20..28]
    {
        Some(fixed)
    } else {
        None
    };
    if let Some(offset) = offset {
        DAMAGE_OFFSET = offset;
        skyline::install_hook!(damage_hook);
        trace("received-damage hook installed");
    } else {
        trace("received-damage hook unavailable; percent tracking active");
    }
}
