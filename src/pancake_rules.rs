use crate::rules::Rank;
pub fn gameplay_reset(ready:bool,training:bool,stock_reset:bool)->bool {stock_reset || !(ready || training)}
pub fn spacing_margin(general:f32,run_stop:f32,running_stop:bool)->f32 {
 let v=if running_stop {run_stop}else{general};
 if v.is_finite() {v.clamp(0.,8.)}else if running_stop {7.}else{5.}
}
#[derive(Clone,Copy)]
pub struct LandingContext {pub aerial:u8,pub directional:bool,pub landing_aerial:u8,pub landing_directional:bool,airborne:bool,age:u8}
impl LandingContext {
 pub const fn new()->Self {Self{aerial:0,directional:false,landing_aerial:0,landing_directional:false,airborne:false,age:0}}
 pub fn observe(&mut self,airborne:bool,aerial:u8,directional:bool,landing:bool) {
  if airborne {
   if !self.airborne {*self=Self::new();}
   if directional {self.directional=true;self.aerial=0;}
   else if aerial!=0 {self.aerial=aerial;self.directional=false;self.age=0;}
   else {self.age=self.age.saturating_add(1);if self.age>8 {self.aerial=0;}}
   self.landing_aerial=0;self.landing_directional=false;
  } else if self.airborne {
   // Ground contact may be visible one callback before the landing status.
   self.landing_aerial=self.aerial;self.landing_directional=self.directional;self.age=0;
  } else if !landing {self.age=self.age.saturating_add(1);if self.age>2 {*self=Self::new();}}
  self.airborne=airborne;
 }
 pub fn landing_name(&self)->&'static str {if self.landing_directional {"landing_fall_special"}else{match self.landing_aerial {1=>"landing_air_n",2=>"landing_air_f",3=>"landing_air_b",_=>""}}}
}
pub fn eligible_window(motion:&str,frame:f32,directional:bool,grounded:bool,whole_intangible:bool,crouching:bool)->Option<Rank> {
 if !grounded || whole_intangible {return None;}
 if crouching {return Some(Rank::Good);}
 window(motion,frame,directional)
}
pub fn window(motion:&str,frame:f32,directional_landing:bool)->Option<Rank> {
 if !frame.is_finite() || frame<0. {return None;}
 match motion {
  "attack_lw3" if frame>=1.=>Some(Rank::Good),
  "squat"|"squat_wait"|"squat_wait2"|"squat_wait3"|"attack_hi4_hold"=>Some(Rank::Good),
  "attack_hi4" if frame>=2. && frame<=8.=>Some(Rank::Good),
  "run_brake_l"|"run_brake_r" if frame>=7. && frame<=16.=>Some(Rank::Great),
  "landing_air_b" if frame<=9.=>Some(Rank::Great),
  "landing_air_f" if frame>=2.=>Some(Rank::Great),
  "landing_air_n" if frame>=1.=>Some(Rank::Great),
  "special_hi" if frame>=9. && frame<=18.=>Some(Rank::Excellent),
  "jump_squat" if frame>=1. && frame<=3.=>Some(Rank::Excellent),
  "landing_fall_special" if directional_landing && frame>=1. && frame<=11.=>Some(Rank::Excellent),
  _=>None,
 }
}
pub fn priority(r:Rank)->u8 {match r {Rank::Good=>1,Rank::Great=>2,Rank::Excellent=>3}}
pub fn body_shape(center:[f32;3],axes:[[f32;3];3])->([f32;3],[f32;3],f32) {
 // Each vector is the animated transform of one body-sphere radius.
 // The Hip rotates local X into world Y; using local Y alone misses squash.
 let extent=|row:usize| (axes[0][row]*axes[0][row]+axes[1][row]*axes[1][row]+axes[2][row]*axes[2][row]).sqrt();
 let height=extent(1).max(0.5);let width=extent(0).max(0.5);
 let radius=height.min(width);let side=(width-radius).max(0.);
 ([center[0]-side,center[1],center[2]],[center[0]+side,center[1],center[2]],radius)
}
#[derive(Clone,Copy)]
pub struct Session {pub rank:Option<Rank>,damage:f32,quiet:u8,age:u16}
impl Session {
 pub const fn new()->Self {Self{rank:None,damage:0.,quiet:0,age:0}}
 pub fn observe(&mut self,rank:Option<Rank>,damage:f32,failed:bool,low:bool)->Option<Rank> {
  if failed || !damage.is_finite() || (self.rank.is_some() && damage>self.damage+0.0001) {*self=Self::new();return None;}
  if let Some(r)=rank {
   if self.rank.is_none() {self.damage=damage;self.age=0;}
   if self.rank.map(|old|priority(r)>priority(old)).unwrap_or(true) {self.rank=Some(r);}
   self.quiet=0;
  } else if self.rank.is_some() {self.quiet=self.quiet.saturating_add(1);}
  if self.rank.is_some() {self.age=self.age.saturating_add(1);}
  // A short gap joins consecutive low animations. Sustained safe crouching
  // settles after the threat has passed rather than waiting for stand-up.
  if self.age>=180 {*self=Self::new();return None;}
  if self.rank.is_some() && ((!low && self.quiet>=6) || self.quiet>=12) {
   let result=self.rank;*self=Self::new();return result;
  }
  None
 }
}
pub fn overhead(a:[f32;3],b:[f32;3],radius:f32,standing:([f32;3],[f32;3],f32),low:([f32;3],[f32;3],f32))->bool {
 overhead_with_margin(a,b,radius,standing,low,0.)
}
pub fn overhead_with_margin(a:[f32;3],b:[f32;3],radius:f32,standing:([f32;3],[f32;3],f32),low:([f32;3],[f32;3],f32),margin:f32)->bool {
 use crate::evasion_rules::capsules_overlap;
 let margin=if margin.is_finite() {margin.clamp(0.,8.)}else{5.};
 let bottom=a[1].min(b[1])-radius;
 let low_top=low.0[1].max(low.1[1])+low.2;
 // Expand only the horizontal recognition corridor. The incoming attack
 // must still clear the real low body, with its original size and height.
 let near=[-margin,-margin*0.5,0.,margin*0.5,margin].iter().any(|dx| {
  let mut p=standing.0;let mut q=standing.1;p[0]+=dx;q[0]+=dx;
  capsules_overlap(a,b,radius,p,q,standing.2)
 });
 bottom>low_top+0.15 && near
  && !capsules_overlap(a,b,radius,low.0,low.1,low.2)
}
