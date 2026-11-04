use crate::rules::Rank;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dodge {
    Spot,
    Roll,
}
pub fn waiting_on_dodge(in_dodge: bool, seen_intangible: bool, intangible: bool) -> bool {
    in_dodge && (!seen_intangible || intangible)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Threat {
    pub opponent: usize,
    pub object: usize,
    pub group: u64,
    pub epoch: u64,
}
#[derive(Clone, Copy)]
pub struct Candidate {
    pub threat: Threat,
    pub rank: Rank,
}
#[derive(Clone, Copy)]
pub struct Session {
    pub pending: Option<Candidate>,
    mode: Option<Dodge>,
    damage: f32,
    failed: bool,
    blocked: bool,
    age: u16,
}
impl Session {
    pub const fn new() -> Self {
        Self {
            pending: None,
            mode: None,
            damage: 0.,
            failed: false,
            blocked: false,
            age: 0,
        }
    }
    pub fn begin(&mut self, mode: Dodge, damage: f32) {
        *self = Self {
            mode: Some(mode),
            damage,
            ..Self::new()
        };
    }
    pub fn threat(&mut self, threat: Threat, multi: bool, intangible: bool, overlap: bool) {
        if !intangible || !overlap || self.failed {
            return;
        }
        let rank = match (self.mode, multi) {
            (Some(Dodge::Spot), true) => {
                self.blocked = true;
                self.pending = None;
                return;
            }
            (Some(Dodge::Spot), false) if !self.blocked => Rank::Great,
            (Some(Dodge::Roll), true) => Rank::Excellent,
            _ => return,
        };
        if self.pending.is_none() {
            self.pending = Some(Candidate { threat, rank });
        }
    }
    pub fn finish(
        &mut self,
        dodging: bool,
        damage: f32,
        hit_or_captured: bool,
        resolved: bool,
    ) -> Option<Rank> {
        self.age = self.age.saturating_add(1);
        self.failed |= !damage.is_finite() || damage > self.damage + 0.0001 || hit_or_captured;
        if self.failed || self.age > 180 {
            self.pending = None;
        }
        if !dodging {
            if self.pending.is_none() {
                *self = Self::new();
            } else if resolved {
                let rank = self.pending.map(|p| p.rank);
                *self = Self::new();
                return rank;
            }
        }
        None
    }
}
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn capsules_overlap(
    p: [f32; 3],
    q: [f32; 3],
    r: f32,
    a: [f32; 3],
    b: [f32; 3],
    s: f32,
) -> bool {
    if !p
        .iter()
        .chain(q.iter())
        .chain(a.iter())
        .chain(b.iter())
        .all(|x| x.is_finite())
        || !r.is_finite()
        || !s.is_finite()
        || r < 0.
        || s < 0.
    {
        return false;
    }
    let u = sub(q, p);
    let v = sub(b, a);
    let w = sub(p, a);
    let uu = dot(u, u);
    let vv = dot(v, v);
    let uv = dot(u, v);
    let uw = dot(u, w);
    let vw = dot(v, w);
    let (mut x, mut y);
    if uu < 0.00001 && vv < 0.00001 {
        x = 0.;
        y = 0.;
    } else if uu < 0.00001 {
        x = 0.;
        y = (vw / vv).clamp(0., 1.);
    } else if vv < 0.00001 {
        y = 0.;
        x = (-uw / uu).clamp(0., 1.);
    } else {
        let den = uu * vv - uv * uv;
        x = if den > 0.00001 {
            ((uv * vw - vv * uw) / den).clamp(0., 1.)
        } else {
            0.
        };
        y = (uv * x + vw) / vv;
        if y < 0. {
            y = 0.;
            x = (-uw / uu).clamp(0., 1.);
        } else if y > 1. {
            y = 1.;
            x = ((uv - uw) / uu).clamp(0., 1.);
        }
    }
    let d = [
        w[0] + u[0] * x - v[0] * y,
        w[1] + u[1] * x - v[1] * y,
        w[2] + u[2] * x - v[2] * y,
    ];
    dot(d, d) <= (r + s) * (r + s)
}
