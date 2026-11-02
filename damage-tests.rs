#![allow(dead_code)]
#[path = "source/src/damage_state.rs"] mod state;
use state::*;
#[test] fn decimals_keep_fractional_damage() {
    assert_eq!(damage_text(26.5).unwrap(), "26.5"); assert_eq!(damage_text(26.05).unwrap(), "26.05");
    assert_eq!(damage_text(26.0).unwrap(), "26"); assert_eq!(damage_text(0.07).unwrap(), "0.07");
}
#[test] fn invalid_numbers_do_not_render() { for x in [0.0,-1.0,f32::NAN,f32::INFINITY] { assert!(damage_text(x).is_none()); } }
#[test] fn actual_damage_includes_game_multipliers() { assert_eq!(applied_damage(10.0,28.0,100.0,118.0),Some(18.0)); }
#[test] fn engine_counter_handles_percent_cap() { assert_eq!(applied_damage(999.0,999.0,100.0,101.5),Some(1.5)); }
#[test] fn healing_and_no_damage_do_not_render() {
    assert_eq!(applied_damage(30.0,26.0,20.0,20.0),None); assert_eq!(applied_damage(26.0,26.0,20.0,20.0),None);
    assert_eq!(applied_damage(f32::NAN,26.0,20.0,20.0),None);
}
#[test] fn throws_use_received_damage_not_hitbox_power() {
    let mut sample=DamageSample::new();assert_eq!(sample.observe(1,0.0,false),None);
    assert_eq!(applied_damage(0.0,1.3,0.0,1.3),Some(1.3));sample.observe(1,1.3,false);
    assert_eq!(sample.observe(1,1.3,false),None);
    assert!((applied_damage(1.3,13.3,1.3,13.3).unwrap()-12.0).abs()<0.00001);
}
#[test] fn periodic_ticks_are_not_duplicated() {
    let mut s=DamageSample::new();s.observe(1,20.0,false);
    assert_eq!(s.observe(1,20.5,false),Some(0.5));assert_eq!(s.observe(1,20.5,false),None);
    s.observe(1,25.5,false);assert_eq!(s.observe(1,25.5,false),None);
}
#[test] fn new_fighters_and_stock_resets_do_not_create_fake_damage() {
    let mut s=DamageSample::new();assert_eq!(s.observe(1,100.0,false),None);
    assert_eq!(s.observe(2,150.0,false),None);assert_eq!(s.observe(2,0.0,true),None);
    assert_eq!(s.observe(2,0.0,false),None);assert_eq!(s.observe(2,4.5,false),Some(4.5));
}
#[test] fn attribution_is_a_bitmask_for_all_eight_players() {
    for id in 0..8 { assert_eq!(single_attacker(1<<id),Some(id)); }
    assert_eq!(single_attacker(0),None);assert_eq!(single_attacker(3),None);assert_eq!(single_attacker(256),None);
}
#[test] fn two_victims_have_independent_damage_samples() {
    let mut a=DamageSample::new();let mut b=DamageSample::new();a.observe(1,0.0,false);b.observe(2,50.0,false);
    assert_eq!(a.observe(1,3.0,false),Some(3.0));assert_eq!(b.observe(2,50.0,false),None);assert_eq!(b.observe(2,60.0,false),Some(10.0));
}

#[path="source/src/damage_layout.rs"] mod damage_layout;
#[test] fn damage_glyphs_are_centered_and_decimal_has_its_own_space() {
 for text in ["40","17.3","0.07","999.99"] {
  let p=damage_layout::positions(text);assert_eq!(p.len(),text.len());
  for pair in p.windows(2) {let right=pair[0].1+damage_layout::width(pair[0].0).unwrap()/2.;let left=pair[1].1-damage_layout::width(pair[1].0).unwrap()/2.;assert!(left-right>0.249);}
  let first=p.first().unwrap();let last=p.last().unwrap();let left=first.1-damage_layout::width(first.0).unwrap()/2.;let right=last.1+damage_layout::width(last.0).unwrap()/2.;assert!((left+right).abs()<0.0001);
 }
}

#[test] fn forward_air_displays_one_actual_total_on_third_hit() {let mut m=MultiHit::new();let k=(1,10,20);assert_eq!(m.add(k,4.8,false),None);assert_eq!(m.add(k,4.8,false),None);let total=m.add(k,7.2,true).unwrap();assert!((total-16.8).abs()<0.0001);assert_eq!(m.flush_unless(None),None);}
#[test] fn down_air_accumulates_five_hits_and_the_finisher() {let mut m=MultiHit::new();let k=(1,10,20);for _ in 0..5 {assert_eq!(m.add(k,1.56,false),None);}assert!((m.add(k,2.4,true).unwrap()-10.2).abs()<0.0001);}
#[test] fn interrupted_multihit_reports_only_damage_landed_once() {let mut m=MultiHit::new();let k=(1,10,20);m.add(k,4.8,false);assert_eq!(m.flush_unless(Some(k)),None);assert_eq!(m.flush_unless(None),Some(4.8));assert_eq!(m.flush_unless(None),None);}
#[test] fn multihit_totals_do_not_mix_opponents_or_next_move() {let mut a=MultiHit::new();let mut b=MultiHit::new();a.add((1,10,20),4.8,false);b.add((1,10,30),4.8,false);assert_eq!(a.flush_unless(Some((2,10,20))),Some(4.8));assert!((b.add((1,10,30),7.2,true).unwrap()-12.).abs()<0.0001);}

#[test] fn a_single_multihit_connection_is_visible_before_any_finisher() {let mut m=MultiHit::new();assert_eq!(m.add((1,10,20),4.8,false),None);assert_eq!(m.total(),4.8);assert_eq!(m.add((1,10,20),4.8,false),None);assert_eq!(m.total(),9.6);}
#[test] fn delayed_percent_updates_do_not_disappear_or_count_twice() {let mut s=DamageSample::new();s.observe(20,10.,false);assert_eq!(callback_damage(10.,10.,0.,6.),None);assert_eq!(s.observe(20,16.,false),Some(6.));assert_eq!(s.observe(20,16.,false),None);assert_eq!(callback_damage(999.,999.,1.,7.),Some(6.));}

#[test] fn rapid_jab_keeps_one_total_through_loop_rewinds_and_finisher() {let mut j=JabSequence::new();let token=j.observe(true,10).unwrap();let mut m=MultiHit::new();m.add((token,10,20),1.2,false);for epoch in [11,12,20,21] {let t=j.observe(true,epoch).unwrap();assert_eq!(t,token);m.flush_unless(Some((t,10,20)));m.add((t,10,20),1.2,false);}assert!((m.total()-6.).abs()<0.0001);assert_eq!(j.observe(false,22),None);assert_ne!(j.observe(true,23),Some(token));}
#[test] fn dedede_up_air_sums_each_actual_connection() {let mut m=MultiHit::new();for _ in 0..7 {m.add((1,10,20),1.2,false);}assert!((m.add((1,10,20),6.,true).unwrap()-14.4).abs()<0.0001);}
#[test] fn metaknight_three_hit_air_combo_keeps_fractional_damage() {let mut m=MultiHit::new();m.add((1,10,20),2.4,false);m.add((1,10,20),2.4,false);assert!((m.add((1,10,20),3.6,true).unwrap()-8.4).abs()<0.0001);}
