#![allow(dead_code)]
#[path="source/src/rules.rs"] mod rules;
#[path="source/src/evasion_rules.rs"] mod evasion_rules;
#[path="source/src/pancake_rules.rs"] mod pancake_rules;
#[path="source/src/native_vector.rs"] mod native_vector;
use rules::Rank;use pancake_rules::*;
#[path="pancake-recorded-tests.rs"] mod recorded_motions;
#[test] fn directional_dodge_survives_a_fall_transition_and_ground_contact() {
 let mut c=LandingContext::new();c.observe(true,0,true,false);c.observe(true,0,false,false);c.observe(false,0,false,false);c.observe(false,0,false,true);assert_eq!(c.landing_name(),"landing_fall_special");
}
#[test] fn each_supported_aerial_reaches_generic_landing() {for (a,name) in [(1,"landing_air_n"),(2,"landing_air_f"),(3,"landing_air_b")] {let mut c=LandingContext::new();c.observe(true,a,false,false);c.observe(true,0,false,false);c.observe(false,0,false,true);assert_eq!(c.landing_name(),name);}}
#[test] fn landing_does_not_reuse_a_long_finished_aerial() {let mut c=LandingContext::new();c.observe(true,1,false,false);for _ in 0..9 {c.observe(true,0,false,false);}c.observe(false,0,false,true);assert_eq!(c.landing_name(),"");}
#[test] fn a_new_aerial_replaces_old_air_dodge_context() {let mut c=LandingContext::new();c.observe(true,0,true,false);c.observe(true,3,false,false);c.observe(false,0,false,true);assert_eq!(c.landing_name(),"landing_air_b");}
#[test] fn later_ordinary_ground_movement_clears_the_landing_context() {let mut c=LandingContext::new();c.observe(true,0,true,false);c.observe(false,0,false,true);for _ in 0..3 {c.observe(false,0,false,false);}assert_eq!(c.landing_name(),"");}
#[test] fn training_can_run_feedback_without_a_versus_ready_go_flag() {assert!(!gameplay_reset(false,true,false));assert!(!gameplay_reset(true,false,false));assert!(gameplay_reset(false,false,false));}
#[test] fn training_still_clears_feedback_on_stock_and_entry_resets() {assert!(gameplay_reset(false,true,true));assert!(gameplay_reset(true,true,true));}
#[test] fn native_crouch_state_does_not_require_a_specific_motion_or_part_zero() {assert_eq!(eligible_window("costume_crouch",0.,false,true,false,true),Some(Rank::Good));}
#[test] fn whole_body_invulnerability_and_airborne_crouch_do_not_count() {assert_eq!(eligible_window("squat_wait",4.,false,true,true,true),None);assert_eq!(eligible_window("squat_wait",4.,false,false,false,true),None);}
#[test] fn native_motion_still_selects_the_other_pancake_grades() {assert_eq!(eligible_window("run_brake_l",8.,false,true,false,false),Some(Rank::Great));assert_eq!(eligible_window("jump_squat",2.,false,true,false,false),Some(Rank::Excellent));}
#[test] fn good_mechanics() {for name in ["attack_lw3","squat","squat_wait","attack_hi4_hold"] {assert_eq!(window(name,4.,false),Some(Rank::Good));}}
#[test] fn great_mechanics() {for (name,f) in [("run_brake_r",7.),("run_brake_l",16.),("landing_air_b",9.),("landing_air_f",2.),("landing_air_n",1.)] {assert_eq!(window(name,f,false),Some(Rank::Great));}}
#[test] fn top_mechanics() {for (name,f,dir) in [("special_hi",9.,false),("special_hi",18.,false),("jump_squat",2.,false),("landing_fall_special",11.,true)] {assert_eq!(window(name,f,dir),Some(Rank::Excellent));}}
#[test] fn outside_notes_windows_stays_quiet() {for (name,f) in [("run_brake_l",6.),("run_brake_r",17.),("special_hi",8.),("special_hi",19.),("jump_squat",4.),("landing_air_b",10.),("attack_air_n",5.)] {assert_eq!(window(name,f,false),None);}}
#[test] fn ordinary_special_landing_is_not_directional_dodge() {assert_eq!(window("landing_fall_special",5.,false),None);}
#[test] fn no_threat_no_reward() {let mut s=Session::new();for _ in 0..200 {assert_eq!(s.observe(None,0.,false,true),None);}}
#[test] fn successful_escape_needs_no_hit_or_followup() {let mut s=Session::new();assert_eq!(s.observe(Some(Rank::Good),0.,false,true),None);for _ in 0..11 {assert_eq!(s.observe(None,0.,false,true),None);}assert_eq!(s.observe(None,0.,false,true),Some(Rank::Good));}
#[test] fn connected_windows_award_highest_once() {let mut s=Session::new();s.observe(Some(Rank::Good),0.,false,true);s.observe(None,0.,false,false);s.observe(Some(Rank::Excellent),0.,false,true);for _ in 0..11 {assert_eq!(s.observe(None,0.,false,true),None);}assert_eq!(s.observe(None,0.,false,true),Some(Rank::Excellent));assert_eq!(s.observe(None,0.,false,false),None);}
#[test] fn damage_or_capture_cancels() {for (d,f) in [(1.,false),(0.,true)] {let mut s=Session::new();s.observe(Some(Rank::Great),0.,false,true);assert_eq!(s.observe(None,d,f,false),None);for _ in 0..20 {assert_eq!(s.observe(None,d,false,false),None);}}}
fn standing()->([f32;3],[f32;3],f32) {([0.,4.,0.],[0.,8.,0.],3.)}
fn low()->([f32;3],[f32;3],f32) {([-2.,2.,0.],[2.,2.,0.],2.)}
#[test] fn run_stop_gets_its_own_wider_edge_recognition() {for x in [-11.,11.] {let p=[x,7.,0.];assert!(!overhead_with_margin(p,p,1.,standing(),low(),spacing_margin(5.,7.,false)));assert!(overhead_with_margin(p,p,1.,standing(),low(),spacing_margin(5.,7.,true)));}}
#[test] fn run_stop_range_keeps_other_routes_and_low_attack_rejection() {assert_eq!(spacing_margin(5.,7.,false),5.);assert_eq!(spacing_margin(5.,7.,true),7.);assert!(!overhead_with_margin([0.,3.,0.],[0.,3.,0.],1.,standing(),low(),spacing_margin(5.,7.,true)));assert!(!overhead_with_margin([20.,7.,0.],[20.,7.,0.],1.,standing(),low(),spacing_margin(5.,7.,true)));}
#[test] fn outer_edge_passes_get_two_more_units_of_horizontal_tolerance() {for x in [-9.,9.] {let p=[x,7.,0.];assert!(!overhead_with_margin(p,p,1.,standing(),low(),3.));assert!(overhead_with_margin(p,p,1.,standing(),low(),5.));}}
#[test] fn wider_edge_tolerance_still_rejects_low_and_distant_misses() {for p in [[0.,3.,0.],[-20.,7.,0.],[20.,7.,0.],[0.,20.,0.]] {assert!(!overhead_with_margin(p,p,1.,standing(),low(),5.));}}
#[test] fn spaced_overhead_threat_gets_horizontal_tolerance() {let p=[7.,7.,0.];assert!(!overhead(p,p,1.,standing(),low()));assert!(overhead_with_margin(p,p,1.,standing(),low(),3.));}
#[test] fn spacing_tolerance_applies_on_both_sides() {for x in [-7.,7.] {let p=[x,7.,0.];assert!(overhead_with_margin(p,p,1.,standing(),low(),3.));}}
#[test] fn spacing_tolerance_does_not_count_low_or_distant_attacks() {for p in [[0.,3.,0.],[20.,7.,0.],[0.,20.,0.]] {assert!(!overhead_with_margin(p,p,1.,standing(),low(),3.));}}
#[test] fn zero_margin_preserves_original_behavior() {for p in [[0.,7.,0.],[7.,7.,0.],[0.,3.,0.]] {assert_eq!(overhead_with_margin(p,p,1.,standing(),low(),0.),overhead(p,p,1.,standing(),low()));}}
// Recorded native animation transforms: body-animation-evidence.json, Bust.
fn recorded_crouch()->([f32;3],[f32;3],f32) {body_shape([0.,1.9500023,0.],[[-0.43953594,2.4870871,0.25376496],[-4.9539227,-0.29184467,2.860153],[2.8749993,0.,4.9796316]])}
#[test] fn crouch_uses_hip_inherited_squash_not_unsquashed_rot() {
 let b=recorded_crouch();assert!(b.0[1]+b.2<4.5);assert!(b.0[0]< -3.);assert!(b.1[0]>3.);
}
#[test] fn ganondorf_grab_clears_recorded_kirby_crouch() {
 let stand=([0.,5.,0.],[0.,5.,0.],5.);
 assert!(overhead([-3.,9.,0.],[3.,9.,0.],4.,stand,recorded_crouch()));
}
#[test] fn ganondorf_jab_clears_recorded_kirby_crouch() {
 let stand=([0.,5.,0.],[0.,5.,0.],5.);
 assert!(overhead([0.,12.,0.],[0.,12.,0.],4.4,stand,recorded_crouch()));
}
#[test] fn ganondorf_grab_does_not_count_when_out_of_reach() {
 assert!(!overhead([20.,9.,0.],[26.,9.,0.],4.,([0.,5.,0.],[0.,5.,0.],5.),recorded_crouch()));
}
#[test] fn overhead_projectile_or_special_sphere_counts() {assert!(overhead([0.,7.,0.],[0.,7.,0.],1.,standing(),low()));}
#[test] fn capsule_crossing_standing_space_counts() {assert!(overhead([-10.,7.,0.],[10.,7.,0.],1.,standing(),low()));}
#[test] fn low_or_far_or_too_high_attacks_do_not_count() {for p in [[0.,3.,0.],[20.,7.,0.],[0.,20.,0.]] {assert!(!overhead(p,p,1.,standing(),low()));}}
#[test] fn invalid_geometry_does_not_count() {assert!(!overhead([f32::NAN,7.,0.],[0.,7.,0.],1.,standing(),low()));}

