use super::*;
#[test] fn recorded_attack_lw3_clears_an_overhead_attack_and_selects_good() {
 let center=[-3.256439413235057e-06, 1.9500000476837158, 2.3511679501098115e-06];let shape=body_shape(center,[[-0.43165020644664764, 2.4974657595157623, 0.37650514394044876], [-5.303352475166321, -0.3095961734652519, 2.1339119970798492], [2.179564982652664, -0.2515856549143791, 5.2708035707473755]]);
 assert_eq!(eligible_window("attack_lw3",2.0,false,true,false,false),Some(Rank::Good));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_attack_hi4_clears_an_overhead_attack_and_selects_good() {
 let center=[8.579258974350523e-07, 3.466136932373047, -3.021052122116089];let shape=body_shape(center,[[-2.3449209332466125, 3.009895086288452, 1.0279136896133423], [-4.309588372707367, -1.9370155036449432, 2.576000690460205], [2.80645489692688, 0.2120351418852806, 5.043937563896179]]);
 assert_eq!(eligible_window("attack_hi4",3.0,false,true,false,false),Some(Rank::Good));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_attack_hi4_hold_clears_an_overhead_attack_and_selects_good() {
 let center=[-1.9992976376670413e-06, 3.466381311416626, -3.063499927520752];let shape=body_shape(center,[[-2.506628632545471, 2.9429668188095093, 1.0101822763681412], [-4.417119920253754, -2.062810957431793, 1.9667598605155945], [2.267112135887146, 0.06263529881834984, 5.232990384101868]]);
 assert_eq!(eligible_window("attack_hi4_hold",2.0,false,true,false,false),Some(Rank::Good));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_run_brake_r_clears_an_overhead_attack_and_selects_great() {
 let center=[0.5873232483863831, 2.775221347808838, -1.5752618312835693];let shape=body_shape(center,[[-1.6932745277881622, 4.0656232833862305, -0.7023800909519196], [-4.381161332130432, -0.9636051952838898, 2.700672149658203], [2.2785352170467377, 1.1685162037611008, 4.670875370502472]]);
 assert_eq!(eligible_window("run_brake_r",13.0,false,true,false,false),Some(Rank::Great));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_run_brake_l_clears_an_overhead_attack_and_selects_great() {
 let center=[0.5883061289787292, 2.6499228477478027, -1.5735681056976318];let shape=body_shape(center,[[-1.6920919716358185, 4.066568613052368, -0.7022885978221893], [-4.380946457386017, -0.9628061205148697, 2.700984477996826], [2.279265969991684, 1.168360710144043, 4.670156836509705]]);
 assert_eq!(eligible_window("run_brake_l",13.0,false,true,false,false),Some(Rank::Great));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_landing_air_b_clears_an_overhead_attack_and_selects_great() {
 let center=[-0.2652132511138916, 1.568894386291504, -2.6908113956451416];let shape=body_shape(center,[[-2.6051172614097595, 1.3328149914741516, 4.545484781265259], [-4.785413146018982, -1.6265146434307098, -1.0496238619089127], [1.9268308579921722, -2.244229018688202, 3.450082540512085]]);
 assert_eq!(eligible_window("landing_air_b",3.0,false,true,false,false),Some(Rank::Great));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_landing_air_f_clears_an_overhead_attack_and_selects_great() {
 let center=[0.1319715827703476, 2.0792317390441895, -0.23285502195358276];let shape=body_shape(center,[[1.0834424197673798, 2.649054229259491, 0.9486240893602371], [3.2097944617271423, -0.7079882919788361, 4.034693539142609], [4.137753248214722, -0.12428484857082367, -3.374873995780945]]);
 assert_eq!(eligible_window("landing_air_f",8.0,false,true,false,false),Some(Rank::Great));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_landing_air_n_clears_an_overhead_attack_and_selects_great() {
 let center=[1.5050172805786133e-06, 1.7916988134384155, -1.5348196029663086e-06];let shape=body_shape(center,[[-2.4120794236660004, 2.00713112950325, 0.7532161474227905], [-4.883039593696594, -0.8857893943786621, 2.7948322892189026], [2.853800654411316, 0.178391020745039, 5.4268479347229]]);
 assert_eq!(eligible_window("landing_air_n",3.0,false,true,false,false),Some(Rank::Great));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_jump_squat_clears_an_overhead_attack_and_selects_excellent() {
 let center=[0.6878181099891663, 2.3589136600494385, 0.056364402174949646];let shape=body_shape(center,[[-0.32242700457572937, 2.2549086809158325, 0.1538468897342682], [-4.083138108253479, -0.7670288532972336, 2.6824551820755005], [3.907385766506195, 0.1498889084905386, 5.990545749664307]]);
 assert_eq!(eligible_window("jump_squat",3.0,false,true,false,false),Some(Rank::Excellent));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_landing_fall_special_clears_an_overhead_attack_and_selects_excellent() {
 let center=[1.0743810889835004e-05, 1.5289191007614136, 0.2883877754211426];let shape=body_shape(center,[[-5.044203996658325, -0.43268632143735886, 4.770636558532715], [0.08695380762219429, -1.9201652705669403, -0.0822116807103157], [4.435614049434662, 6.39199015495251e-05, 4.68997597694397]]);
 assert_eq!(eligible_window("landing_fall_special",5.0,true,true,false,false),Some(Rank::Excellent));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}
#[test] fn recorded_special_hi_clears_an_overhead_attack_and_selects_excellent() {
 let center=[-0.2679513096809387, 3.3560380935668945, -3.0104730129241943];let shape=body_shape(center,[[0.3983699157834053, 2.8271496295928955, 2.726036310195923], [-4.366025030612946, -0.8162429928779602, 3.0561941862106323], [3.321012854576111, -1.4122165739536285, 3.6908763647079468]]);
 assert_eq!(eligible_window("special_hi",13.0,false,true,false,false),Some(Rank::Excellent));
 assert!(overhead([center[0],12.,center[2]],[center[0],12.,center[2]],4.4,([0.,5.,0.],[0.,5.,0.],5.),shape),"Recorded animation must clear the overhead jab");
}