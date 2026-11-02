use std::sync::Mutex;

pub type MoveKey=(u64,usize,usize);
#[derive(Clone,Copy)]
pub struct MultiHit {pub key:Option<MoveKey>,total:f32}
impl MultiHit {
    pub const fn new()->Self {Self{key:None,total:0.0}}
    pub fn flush_unless(&mut self,key:Option<MoveKey>)->Option<f32> {
        if self.key==key {return None;}let damage=self.total;*self=Self::new();if damage>0.0 {Some(damage)}else{None}
    }
    pub fn total(&self)->f32 {self.total}
    pub fn add(&mut self,key:MoveKey,damage:f32,finisher:bool)->Option<f32> {
        self.key=Some(key);if damage.is_finite() && damage>0.0 {self.total+=damage;}
        if finisher {let sum=self.total;*self=Self::new();if sum>0.0 {Some(sum)}else{None}}else{None}
    }
}
pub static MULTI:Mutex<[[MultiHit;8];8]>=Mutex::new([[MultiHit::new();8];8]);
#[derive(Clone,Copy)]
pub struct DamageOwner {pub source:u32,pub target:usize,pub age:u8}
pub static OWNERS:Mutex<[Option<DamageOwner>;8]>=Mutex::new([None;8]);
pub static OBJECT_IDS:Mutex<[Option<u32>;8]>=Mutex::new([None;8]);
#[derive(Clone,Copy)]
pub struct JabSequence {active:bool,token:u64}
impl JabSequence {
    pub const fn new()->Self {Self{active:false,token:0}}
    pub fn observe(&mut self,jab:bool,epoch:u64)->Option<u64> {
        if !jab {self.active=false;return None;}
        if !self.active {self.token=epoch;self.active=true;}Some(self.token)
    }
}
pub static JABS:Mutex<[JabSequence;8]>=Mutex::new([JabSequence::new();8]);

#[derive(Clone, Copy)]
pub struct DamageSample {
    object: usize,
    percent: f32,
    ready: bool,
}
impl DamageSample {
    pub const fn new() -> Self { Self { object: 0, percent: 0.0, ready: false } }
    pub fn observe(&mut self, object: usize, percent: f32, reset: bool) -> Option<f32> {
        let previous = self.percent;
        let valid = self.ready && self.object == object && !reset && percent.is_finite();
        self.object = object;
        self.percent = percent;
        self.ready = !reset && percent.is_finite();
        if valid && percent > previous + 0.0001 { Some(percent - previous) } else { None }
    }
}
pub fn single_attacker(mask: u16) -> Option<usize> {
    if mask.count_ones() == 1 && mask < 256 { Some(mask.trailing_zeros() as usize) } else { None }
}
pub fn damage_text(damage: f32) -> Option<String> {
    if !damage.is_finite() || damage < 0.005 { return None; }
    let text = format!("{:.2}", damage);
    Some(text.trim_end_matches('0').trim_end_matches('.').to_string())
}
pub static DAMAGE: Mutex<[DamageSample; 8]> = Mutex::new([DamageSample::new(); 8]);
pub fn applied_damage(before_percent: f32, after_percent: f32, before_total: f32, after_total: f32) -> Option<f32> {
    let delta = after_percent - before_percent;
    if delta.is_finite() && delta > 0.0001 { return Some(delta); }
    let reported = after_total - before_total;
    if delta == 0.0 && reported.is_finite() && reported > 0.0001 { Some(reported) } else { None }
}
pub fn callback_damage(before:f32,after:f32,before_total:f32,after_total:f32)->Option<f32> {
    if after>before || before>=999.0 {applied_damage(before,after,before_total,after_total)} else {None}
}
