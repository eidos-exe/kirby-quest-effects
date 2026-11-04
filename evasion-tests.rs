#![allow(dead_code)]
#[path = "source/src/rules.rs"]
mod rules;
#[path = "source/src/evasion_rules.rs"]
mod evasion_rules;
use rules::Rank;
use evasion_rules::*;
fn threat() -> Threat {
    Threat {
        opponent: 1,
        object: 10,
        group: 20,
        epoch: 1,
    }
}
#[test]
fn single_hit_spot_dodge_is_great_after_success_not_on_button_press() {
    let mut s = Session::new();
    s.begin(Dodge::Spot, 10.);
    assert_eq!(s.finish(true, 10., false, true), None);
    s.threat(threat(), false, true, true);
    assert_eq!(s.finish(true, 10., false, true), None);
    assert_eq!(s.finish(false, 10., false, true), Some(Rank::Great));
    assert_eq!(s.finish(false, 10., false, true), None);
}
#[test]
fn either_roll_direction_escaping_multihit_is_top_rank() {
    for _direction in [-1, 1] {
        let mut s = Session::new();
        s.begin(Dodge::Roll, 0.);
        s.threat(threat(), true, true, true);
        assert_eq!(s.finish(false, 0., false, true), Some(Rank::Excellent));
    }
}
#[test]
fn spot_dodging_a_multihit_never_gets_a_reward() {
    let mut s = Session::new();
    s.begin(Dodge::Spot, 0.);
    s.threat(threat(), true, true, true);
    s.threat(threat(), false, true, true);
    assert_eq!(s.finish(false, 0., false, true), None);
}
#[test]
fn rolling_a_single_hit_does_not_get_the_multihit_reward() {
    let mut s = Session::new();
    s.begin(Dodge::Roll, 0.);
    s.threat(threat(), false, true, true);
    assert_eq!(s.finish(false, 0., false, true), None);
}
#[test]
fn whiffing_far_away_or_dodging_in_startup_stays_quiet() {
    for (xlu, overlap) in [(true, false), (false, true), (false, false)] {
        let mut s = Session::new();
        s.begin(Dodge::Spot, 0.);
        s.threat(threat(), false, xlu, overlap);
        assert_eq!(s.finish(false, 0., false, true), None);
    }
}
#[test]
fn later_multihit_or_capture_cancels_an_apparent_escape() {
    for (damage, captured) in [(1., false), (0., true)] {
        let mut s = Session::new();
        s.begin(Dodge::Roll, 0.);
        s.threat(threat(), true, true, true);
        assert_eq!(s.finish(false, damage, captured, true), None);
    }
}
#[test]
fn gap_between_hits_does_not_finish_the_escape() {
    let mut s = Session::new();
    s.begin(Dodge::Roll, 0.);
    s.threat(threat(), true, true, true);
    assert_eq!(s.finish(false, 0., false, false), None);
    assert_eq!(s.finish(false, 0., false, true), Some(Rank::Excellent));
}
#[test]
fn repeated_hitboxes_cannot_spam_one_dodge_reward() {
    let mut s = Session::new();
    s.begin(Dodge::Roll, 0.);
    for _ in 0..50 {
        s.threat(threat(), true, true, true);
        assert_eq!(s.finish(true, 0., false, false), None);
    }
    assert_eq!(s.finish(false, 0., false, true), Some(Rank::Excellent));
    assert_eq!(s.finish(false, 0., false, true), None);
}
#[test]
fn only_real_body_space_intersections_count() {
    let a = [0., 2., 0.];
    let b = [0., 6., 0.];
    assert!(capsules_overlap([-5., 4., 0.], [5., 4., 0.], 1., a, b, 2.));
    assert!(!capsules_overlap(
        [15., 4., 0.],
        [20., 4., 0.],
        1.,
        a,
        b,
        2.
    ));
    assert!(!capsules_overlap(
        [-5., 15., 0.],
        [5., 15., 0.],
        1.,
        a,
        b,
        2.
    ));
    assert!(!capsules_overlap(
        [f32::NAN, 4., 0.],
        [5., 4., 0.],
        1.,
        a,
        b,
        2.
    ));
}
#[test]
fn rolled_from_position_is_the_space_the_attack_needed_to_threaten() {
    assert!(capsules_overlap(
        [-1., 4., 0.],
        [1., 4., 0.],
        1.,
        [0., 2., 0.],
        [0., 6., 0.],
        2.
    ));
    assert!(!capsules_overlap(
        [-1., 4., 0.],
        [1., 4., 0.],
        1.,
        [20., 2., 0.],
        [20., 6., 0.],
        2.
    ));
}
#[test]
fn parallel_and_point_capsules_handle_degenerate_hitboxes() {
    assert!(capsules_overlap(
        [0., 0., 0.],
        [0., 0., 0.],
        1.,
        [1., 0., 0.],
        [1., 0., 0.],
        1.
    ));
    assert!(!capsules_overlap(
        [0., 0., 0.],
        [0., 0., 0.],
        1.,
        [3., 0., 0.],
        [3., 0., 0.],
        1.
    ));
    assert!(capsules_overlap(
        [0., 0., 0.],
        [0., 5., 0.],
        1.,
        [1., 0., 0.],
        [1., 5., 0.],
        1.
    ));
}

#[test]
fn pre_invulnerability_frames_keep_the_dodge_session_then_reward_on_window_end() {
    let mut s = Session::new();
    s.begin(Dodge::Spot, 20.);
    assert_eq!(
        s.finish(waiting_on_dodge(true, false, false), 20., false, false),
        None
    );
    s.threat(threat(), false, true, true);
    assert_eq!(
        s.finish(waiting_on_dodge(true, true, true), 20., false, true),
        None
    );
    assert_eq!(
        s.finish(waiting_on_dodge(true, true, false), 20., false, true),
        Some(Rank::Great)
    );
}
#[test]
fn starting_a_new_dodge_after_the_previous_reward_does_not_lose_or_repeat_it() {
    let mut s = Session::new();
    for _ in 0..3 {
        s.begin(Dodge::Spot, 0.);
        s.threat(threat(), false, true, true);
        assert_eq!(s.finish(false, 0., false, true), Some(Rank::Great));
        assert_eq!(s.finish(false, 0., false, true), None);
    }
}
