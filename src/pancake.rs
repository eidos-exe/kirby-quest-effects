use super::*;
use super::pancake_rules::{self,Session};
#[derive(Clone,Copy)]
struct Tracker {session:Session,standing_top:f32,standing_bottom:f32,width:f32,last_motion:u64,directional:bool,queued:Option<Rank>,pending:[u64;16],done:[u64;32],cursor:usize,last_probe:u64,last_status:i32,ticks:u16,context:pancake_rules::LandingContext,last_grade:Option<Rank>}
impl Tracker {const fn new()->Self {Self{session:Session::new(),standing_top:11.,standing_bottom:1.,width:5.,last_motion:0,directional:false,queued:None,pending:[0;16],done:[0;32],cursor:0,last_probe:0,last_status:-1,ticks:0,context:pancake_rules::LandingContext::new(),last_grade:None}}}
static TRACKERS:Mutex<[Tracker;8]>=Mutex::new([Tracker::new();8]);
const MOTIONS:[&str;17]=["attack_lw3","squat","squat_wait","squat_wait2","squat_wait3","attack_hi4_hold","attack_hi4","run_brake_l","run_brake_r","landing_air_b","landing_air_f","landing_air_n","special_hi","jump_squat","landing_fall_special","wait","escape_air_slide"];
unsafe fn point(b:*mut BattleObjectModuleAccessor,bone:u64,v:[f32;3])->[f32;3] {
 let input=native_vector::JointVector::new(v);let mut out=native_vector::JointVector::new([0.;3]);
 let input_ptr=&input as *const native_vector::JointVector as *const Vector3f;
 let out_ptr=&mut out as *mut native_vector::JointVector as *mut Vector3f;
 ModelModule::joint_global_position_with_offset(b,Hash40::new_raw(bone),input_ptr,out_ptr,true);
 out.xyz()
}
unsafe fn body(b:*mut BattleObjectModuleAccessor)->([f32;3],[f32;3],f32) {
 let bone=Hash40::new("bust").hash;
 let center=point(b,bone,[0.;3]);let mut axes=[[0.;3];3];
 for i in 0..3 {let mut offset=[0.;3];offset[i]=5.;let edge=point(b,bone,offset);for j in 0..3 {axes[i][j]=edge[j]-center[j];}}
 pancake_rules::body_shape(center,axes)
}
unsafe fn current_rank(b:*mut BattleObjectModuleAccessor,t:&Tracker)->Option<Rank> {
 if utility::get_kind(&mut *b)!=*FIGHTER_KIND_KIRBY {return None;}
 // Hurtbox index zero may be disabled or absent on a costume; that is not
 // whole-body invulnerability and must not disable the pancake mechanic.
 let status=StatusModule::status_kind(b);
 // Eligible low animations plus geometric clearance establish a pancake.
 // A disabled individual hurtbox is not a reason to reject crouching.
 // For air-dodge landing specifically, wait out its actual XLU window.
 let intangible=(*FIGHTER_STATUS_KIND_ESCAPE..=*FIGHTER_STATUS_KIND_ESCAPE_B).contains(&status)
  || status==*FIGHTER_STATUS_KIND_ESCAPE_AIR
  || (t.directional && HitModule::get_whole(b,0)==*HIT_STATUS_XLU);
 let crouching=status==*FIGHTER_STATUS_KIND_SQUAT || status==*FIGHTER_STATUS_KIND_SQUAT_WAIT;
 let m=MotionModule::motion_kind(b);let native_name=MOTIONS.iter().find(|s|Hash40::new(s).hash==m).copied().unwrap_or("");
 let name=if status==*FIGHTER_STATUS_KIND_ATTACK_LW3 {"attack_lw3"}
  else if status==*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD {"attack_hi4_hold"}
  else if status==*FIGHTER_STATUS_KIND_ATTACK_HI4 {"attack_hi4"}
  else if status==*FIGHTER_STATUS_KIND_RUN_BRAKE {"run_brake_r"}
  else if status==*FIGHTER_STATUS_KIND_JUMP_SQUAT {"jump_squat"}
  else if status==*FIGHTER_STATUS_KIND_SPECIAL_HI {"special_hi"}
  else if [*FIGHTER_STATUS_KIND_LANDING,*FIGHTER_STATUS_KIND_LANDING_LIGHT,*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status) {
   if t.context.landing_directional {"landing_fall_special"}
   else if native_name.starts_with("landing_air_") {native_name}
   else {t.context.landing_name()}
  } else {native_name};
 // Cornell timing windows are elapsed game frames, not accelerated clip frames.
 pancake_rules::eligible_window(name,t.ticks as f32,t.directional,StatusModule::situation_kind(b)==*SITUATION_KIND_GROUND,intangible,crouching)
}
pub fn reset(slot:usize) {TRACKERS.lock().unwrap()[slot]=Tracker::new();}
unsafe fn check(source:*mut BattleObjectModuleAccessor,defender:*mut BattleObjectModuleAccessor,slot:usize,epoch:u64) {
 if source==defender || source.is_null() {return;}
 if TeamModule::team_no(source)==TeamModule::team_no(defender) {return;}
 let mut t=TRACKERS.lock().unwrap()[slot];
 let rank=match current_rank(defender,&t) {Some(r)=>r,None=>return};
 let pos=*PostureModule::pos(defender);let low=body(defender);
 let low_top=low.0[1].max(low.1[1])+low.2;
 if low_top>=pos.y+t.standing_top-0.5 {return;}
 let radius=t.width.min((t.standing_top-t.standing_bottom)*0.5).max(0.5);
 let running_stop=StatusModule::status_kind(defender)==*FIGHTER_STATUS_KIND_RUN_BRAKE
  || [Hash40::new("run_brake_l").hash,Hash40::new("run_brake_r").hash].contains(&MotionModule::motion_kind(defender));
 let margin={let settings=SETTINGS.lock().unwrap();pancake_rules::spacing_margin(settings.pancake_horizontal_margin,settings.pancake_run_stop_margin,running_stop)};
 let standing=([pos.x,pos.y+t.standing_bottom+radius,pos.z],[pos.x,pos.y+t.standing_top-radius,pos.z],radius);
 let probe_key=(((*source).battle_object_id as u64)<<32)^epoch.wrapping_mul(257)^MotionModule::motion_kind(source);
 for id in 0..8 {
  if !AttackModule::is_attack(source,id,false) {continue;}
  let ptr=AttackModule::attack_data(source,id,false);if ptr.is_null() {continue;}
  let d=&*ptr;if !d.power.is_finite() || d.power<=0. || d.target_situation & 1==0 {continue;}
  let a=point(source,d.node,[d.x,d.y,d.z]);
  // Source classifications identify extended capsules; unknowns use their
  // first sphere, avoiding an invented end point from unused native fields.
  let capsule=if entry(source).is_some() {evasion_profiles::find(utility::get_kind(&mut *source),MotionModule::motion_kind(source)).map(|p|p.capsules&(1<<id)!=0).unwrap_or(false)}else{pancake_profiles::weapon_capsules(utility::get_kind(&mut *source))&(1<<id)!=0};
  let b=if capsule {point(source,d.node,[d.x2,d.y2,d.z2])}else{a};
  let scale=PostureModule::scale(source).abs();
  let clears=pancake_rules::overhead_with_margin(a,b,d.size*scale,standing,low,margin);
  if t.last_probe!=probe_key {trace(&format!("pancake attack probe: source_kind={} frame={:.2} center={:?} radius={:.2} low_top={:.2} stand_top={:.2} clears={}",utility::get_kind(&mut *source),MotionModule::frame(source),a,d.size*scale,low_top,pos.y+t.standing_top,clears));t.last_probe=probe_key;}
  if !clears {continue;}
  let key=(((*source).battle_object_id as u64)<<32)^epoch.wrapping_mul(257)^MotionModule::motion_kind(source)^(id as u64+1);
  let key=if key==0 {1}else{key};
  if t.done.contains(&key) {continue;}
  if !t.pending.contains(&key) {if let Some(p)=t.pending.iter_mut().find(|x|**x==0) {*p=key;}else{continue;}}
  if t.queued.map(|r|pancake_rules::priority(rank)>pancake_rules::priority(r)).unwrap_or(true) {t.queued=Some(rank);}
 }
 if entry(source).is_some() {
  let frame=MotionModule::frame(source);
  for grab in pancake_profiles::grabs(utility::get_kind(&mut *source),MotionModule::motion_kind(source)) {
   if frame<grab.start || frame>=grab.end || !GrabModule::is_grab(source,grab.id) {continue;}
   let a=point(source,grab.bone,grab.a);let b=point(source,grab.bone,grab.b);
   let clears=pancake_rules::overhead_with_margin(a,b,grab.radius*PostureModule::scale(source).abs(),standing,low,margin);
   if t.last_probe!=probe_key {trace(&format!("pancake grab probe: source_kind={} frame={:.2} a={:?} b={:?} radius={:.2} low_top={:.2} stand_top={:.2} clears={}",utility::get_kind(&mut *source),frame,a,b,grab.radius,low_top,pos.y+t.standing_top,clears));t.last_probe=probe_key;}
   if !clears {continue;}
   let key=(((*source).battle_object_id as u64)<<32)^epoch.wrapping_mul(257)^MotionModule::motion_kind(source)^(grab.id as u64+129);
   if t.done.contains(&key) {continue;}
   if !t.pending.contains(&key) {if let Some(p)=t.pending.iter_mut().find(|x|**x==0) {*p=key;}else{continue;}}
   if t.queued.map(|r|pancake_rules::priority(rank)>pancake_rules::priority(r)).unwrap_or(true) {t.queued=Some(rank);}
  }
 }
 TRACKERS.lock().unwrap()[slot]=t;
}
pub unsafe fn frame(b:*mut BattleObjectModuleAccessor,slot:usize) {
 if utility::get_kind(&mut *b)!=*FIGHTER_KIND_KIRBY {return;}
 let mut t=TRACKERS.lock().unwrap()[slot];let m=MotionModule::motion_kind(b);let status=StatusModule::status_kind(b);
 let changed=status!=t.last_status || m!=t.last_motion;
 if changed {t.ticks=1;} else if !StopModule::is_stop(b) {t.ticks=t.ticks.saturating_add(1);}
 let airborne=StatusModule::situation_kind(b)==*SITUATION_KIND_AIR;
 let aerial=if status==*FIGHTER_STATUS_KIND_ATTACK_AIR {
  if m==Hash40::new("attack_air_n").hash {1}else if m==Hash40::new("attack_air_f").hash {2}else if m==Hash40::new("attack_air_b").hash {3}else{4}
 }else{0};
 let directional=status==*FIGHTER_STATUS_KIND_ESCAPE_AIR && (m==Hash40::new("escape_air_slide").hash || WorkModule::is_flag(b,*FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE));
 let landing=[*FIGHTER_STATUS_KIND_LANDING,*FIGHTER_STATUS_KIND_LANDING_LIGHT,*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status);
 t.context.observe(airborne,aerial,directional,landing);t.directional=t.context.landing_directional;
 t.last_motion=m;t.last_status=status;
 let grade=current_rank(b,&t);
 if grade!=t.last_grade {
  if let Some(rank)=grade {let p=*PostureModule::pos(b);let shape=body(b);trace(&format!("pancake window: slot={} status={} motion={:x} tick={} grade={:?} low_top={:.2} standing_top={:.2}",slot,status,m,t.ticks,rank,shape.0[1]+shape.2-p.y,t.standing_top));}
  t.last_grade=grade;
 }
 if m==Hash40::new("wait").hash {
  let p=*PostureModule::pos(b);let c=body(b);
  t.standing_top=c.0[1].max(c.1[1])+c.2-p.y;t.standing_bottom=c.0[1].min(c.1[1])-c.2-p.y;
  t.width=((c.1[0]-c.0[0]).abs()*0.5+c.2).max(1.);
 }
 TRACKERS.lock().unwrap()[slot]=t;
 for id in *damage_state::OBJECT_IDS.lock().unwrap() {
  if let Some(id)=id {if sv_battle_object::is_active(id) {let other=sv_battle_object::module_accessor(id);if let Some(a)=sample(other) {check(other,b,slot,a.epoch);}}}
 }
 let mut t=TRACKERS.lock().unwrap()[slot];let queued=t.queued.take();
 let status=StatusModule::status_kind(b);
 let failed=damage_status(status)||(status>=*FIGHTER_STATUS_KIND_CAPTURE_PULLED && status<=*FIGHTER_STATUS_KIND_CAPTURE_JUMP);
 let low=current_rank(b,&t).is_some();
 let had_session=t.session.rank.is_some();
 let reward=t.session.observe(queued,DamageModule::damage(b,0),failed,low);
 if reward.is_some() || failed || (had_session && t.session.rank.is_none()) {
  for key in t.pending {if key!=0 {let cursor=t.cursor%32;t.done[cursor]=key;t.cursor+=1;}}
  t.pending=[0;16];
 }
 TRACKERS.lock().unwrap()[slot]=t;
 if let Some(rank)=reward {announce(b,rank,"successful low-profile pancake escape");}
}
pub unsafe extern "C" fn weapon_frame(agent:&mut smash::lua2cpp::L2CWeaponCommon) {
 if !app::sv_information::is_ready_go() && !app::smashball::is_training_mode() {return;}
 let source=agent.module_accessor;
 let ids=*damage_state::OBJECT_IDS.lock().unwrap();
 for slot in 0..8 {if let Some(id)=ids[slot] {if sv_battle_object::is_active(id) {
  let b=sv_battle_object::module_accessor(id);
  if utility::get_kind(&mut *b)==*FIGHTER_KIND_KIRBY {check(source,b,slot,0);}
 }}}
}
