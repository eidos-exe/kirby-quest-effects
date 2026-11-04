use crate::rules::Rank;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decision {
    Play,
    Interrupt(Rank),
    Skip,
}
#[derive(Clone, Copy)]
pub struct Feedback {
    pub active: Option<Rank>,
    grace: u8,
}
fn priority(rank: Rank) -> u8 {
    match rank {
        Rank::Good => 1,
        Rank::Great => 2,
        Rank::Excellent => 3,
    }
}
impl Feedback {
    pub const fn new() -> Self {
        Self {
            active: None,
            grace: 0,
        }
    }
    pub fn tick(&mut self) {
        self.grace = self.grace.saturating_sub(1);
    }
    pub fn request(&mut self, rank: Rank, playing: bool) -> Decision {
        // Give a newly started sound two callbacks to become visible to the engine.
        if !playing && self.grace == 0 {
            self.active = None;
        }
        let decision = match self.active {
            Some(prior) if priority(rank) > priority(prior) => Decision::Interrupt(prior),
            Some(_) => Decision::Skip,
            None => Decision::Play,
        };
        if decision != Decision::Skip {
            self.active = Some(rank);
            self.grace = 2;
        }
        decision
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_reward_plays_immediately() {
        assert_eq!(Feedback::new().request(Rank::Good, false), Decision::Play);
    }
    #[test]
    fn repeats_never_queue() {
        let mut f = Feedback::new();
        f.request(Rank::Good, false);
        for _ in 0..300 {
            assert_eq!(f.request(Rank::Good, true), Decision::Skip);
            f.tick();
        }
        assert_eq!(f.request(Rank::Good, false), Decision::Play);
    }
    #[test]
    fn all_higher_ranks_interrupt() {
        for (a, b) in [
            (Rank::Good, Rank::Great),
            (Rank::Good, Rank::Excellent),
            (Rank::Great, Rank::Excellent),
        ] {
            let mut f = Feedback::new();
            f.request(a, false);
            assert_eq!(f.request(b, true), Decision::Interrupt(a));
        }
    }
    #[test]
    fn lower_ranks_do_not_interrupt() {
        let mut f = Feedback::new();
        f.request(Rank::Excellent, false);
        for r in [Rank::Good, Rank::Great, Rank::Excellent] {
            assert_eq!(f.request(r, true), Decision::Skip);
        }
    }
    #[test]
    fn finished_clip_allows_any_new_rank() {
        let mut f = Feedback::new();
        f.request(Rank::Excellent, false);
        f.tick();
        f.tick();
        assert_eq!(f.request(Rank::Good, false), Decision::Play);
    }
    #[test]
    fn same_frame_engine_delay_cannot_overlap() {
        let mut f = Feedback::new();
        f.request(Rank::Good, false);
        assert_eq!(f.request(Rank::Good, false), Decision::Skip);
        f.tick();
        assert_eq!(f.request(Rank::Good, false), Decision::Skip);
    }
    #[test]
    fn silence_never_triggers_a_delayed_reward() {
        let mut f = Feedback::new();
        f.request(Rank::Good, false);
        f.request(Rank::Good, true);
        for _ in 0..600 {
            f.tick();
        }
        assert_eq!(f.grace, 0);
        assert_eq!(f.active, Some(Rank::Good));
    }
}
