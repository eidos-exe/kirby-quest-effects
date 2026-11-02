use smash::{lib::lua_const::*,phx::Hash40};
pub struct Grab {pub id:i32,pub bone:u64,pub radius:f32,pub a:[f32;3],pub b:[f32;3],pub start:f32,pub end:f32}
pub fn grabs(kind:i32,motion:u64)->Vec<Grab> {
if kind==*FIGHTER_KIND_BAYONETTA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 9.2],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_BAYONETTA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.9],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_BAYONETTA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -15.5],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_BRAVE && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 7.0],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_BRAVE && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 9.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_BRAVE && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -13.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_BUDDY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.0],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_BUDDY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 11.8],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_BUDDY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -15.4],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_CAPTAIN && motion==Hash40::new("special_hi").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 13.0, 7.0],b:[0.0, 13.0, 7.0],start:14.0,end:31.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.5,a:[0.0, 8.8, 13.7],b:[0.0, 8.8, 13.7],start:14.0,end:31.0},Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 15.0, 6.0],b:[0.0, 15.0, 6.0],start:15.0,end:31.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 11.0, 6.0],b:[0.0, 11.0, 6.0],start:15.0,end:31.0},];}
if kind==*FIGHTER_KIND_CAPTAIN && motion==Hash40::new("special_air_hi").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 13.0, 7.0],b:[0.0, 13.0, 7.0],start:14.0,end:31.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.5,a:[0.0, 8.8, 13.7],b:[0.0, 8.8, 13.7],start:14.0,end:31.0},Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 15.0, 6.0],b:[0.0, 15.0, 6.0],start:15.0,end:31.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 11.0, 6.0],b:[0.0, 11.0, 6.0],start:15.0,end:31.0},];}
if kind==*FIGHTER_KIND_CAPTAIN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_CAPTAIN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 11.6],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_CAPTAIN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -16.2],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_CHROM && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 7.4],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_CHROM && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.25, 4.0],b:[0.0, 7.25, 9.1],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_CHROM && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -12.8],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_CLOUD && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_CLOUD && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_CLOUD && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("special_n_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:9.0,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 7.0],start:17.0,end:1000000.0},Grab{id:0,bone:Hash40::new("top").hash,radius:7.5,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 7.0],start:17.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("special_n_loop").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 9.0, 5.0],b:[0.0, 9.0, 12.0],start:0.0,end:1000000.0},Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 5.5, 5.0],b:[0.0, 5.5, 12.0],start:0.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("special_air_n_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:9.0,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 4.0],start:17.0,end:1000000.0},Grab{id:0,bone:Hash40::new("top").hash,radius:7.5,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 4.0],start:17.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("special_air_n_loop").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 9.0, 5.0],b:[0.0, 9.0, 9.0],start:0.0,end:1000000.0},Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 5.5, 5.0],b:[0.0, 5.5, 9.0],start:0.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.5,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 12.5],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 14.4],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_DEDEDE && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.5,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -19.3],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_DEMON && motion==Hash40::new("special_lw").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.75, 5.0],b:[0.0, 8.75, 5.0],start:18.0,end:20.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 7.75, 12.0],b:[0.0, 7.75, 10.0],start:18.0,end:20.0},Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 14.0, 5.0],b:[0.0, 8.75, 5.0],start:19.0,end:20.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 19.0, 9.0],b:[0.0, 7.75, 10.0],start:19.0,end:20.0},];}
if kind==*FIGHTER_KIND_DEMON && motion==Hash40::new("special_air_lw").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.75, 5.0],b:[0.0, 8.75, 5.0],start:18.0,end:20.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 7.75, 12.0],b:[0.0, 7.75, 10.0],start:18.0,end:20.0},Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 15.0, 5.0],b:[0.0, 8.75, 5.0],start:19.0,end:20.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 20.0, 9.0],b:[0.0, 7.75, 10.0],start:19.0,end:20.0},];}
if kind==*FIGHTER_KIND_DEMON && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 9.0, 7.0],b:[0.0, 9.0, 11.2],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_DEMON && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 6.6, 5.0],b:[0.0, 6.6, 10.7],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_DEMON && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -5.0],b:[0.0, 6.6, -12.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_DEMON && motion==Hash40::new("catch_command").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 6.6, 5.0],b:[0.0, 6.6, 11.2],start:4.0,end:5.0},];}
if kind==*FIGHTER_KIND_DIDDY && motion==Hash40::new("special_air_s_jump").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 2.5, 4.5],b:[0.0, 2.5, 4.5],start:7.0,end:1000000.0},Grab{id:1,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 4.0],start:7.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_DIDDY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.7,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 7.8],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_DIDDY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 9.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_DIDDY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.7,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -13.6],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_DOLLY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 9.0, 5.0],b:[0.0, 9.0, 9.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_DOLLY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 9.0, 5.0],b:[0.0, 9.0, 9.7],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_DOLLY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.2, -5.0],b:[0.0, 7.2, -13.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_DONKEY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.8,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 13.7],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_DONKEY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.6,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 15.9],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_DONKEY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.8,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -21.5],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_DUCKHUNT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 11.2],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_DUCKHUNT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 12.9],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_DUCKHUNT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -18.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_EDGE && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, 4.7],b:[0.0, 8.5, 8.7],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_EDGE && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, 4.7],b:[0.0, 7.0, 12.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_EDGE && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.7],b:[0.0, 7.0, -15.0],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_EFLAME && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, 4.0],b:[0.0, 8.5, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_EFLAME && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.6, 3.0],b:[0.0, 6.6, 11.5],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_EFLAME && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -6.0],b:[0.0, 6.6, -14.7],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_ELIGHT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, 4.0],b:[0.0, 8.5, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_ELIGHT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.6, 3.0],b:[0.0, 6.6, 11.5],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_ELIGHT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -6.0],b:[0.0, 6.6, -14.7],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_FALCO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("trans").hash,radius:3.1,a:[0.0, 7.2, 4.0],b:[0.0, 7.2, 8.9],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_FALCO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("trans").hash,radius:2.5,a:[0.0, 7.2, 4.0],b:[0.0, 6.0, 10.5],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_FALCO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("trans").hash,radius:3.1,a:[0.0, 7.2, -4.0],b:[0.0, 7.2, -14.9],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_FOX && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 8.0],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_FOX && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.4,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 9.6],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_FOX && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -13.5],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_GAMEWATCH && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 9.3],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_GAMEWATCH && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 13.5],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_GAMEWATCH && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 5.0, -4.0],b:[0.0, 5.0, -15.6],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("special_s_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 8.0, 7.5],b:[0.0, 8.0, 7.5],start:16.0,end:31.0},Grab{id:1,bone:Hash40::new("top").hash,radius:1.0,a:[0.0, 8.0, 7.2],b:[0.0, 8.0, 7.2],start:16.0,end:31.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("special_air_s_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 6.0, 7.5],b:[0.0, 6.0, 7.5],start:19.0,end:32.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 11.0, 7.5],b:[0.0, 11.0, 7.5],start:19.0,end:32.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("special_hi").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.4,a:[0.0, 16.0, 6.5],b:[0.0, 16.0, 6.5],start:14.0,end:29.0},Grab{id:1,bone:Hash40::new("top").hash,radius:6.5,a:[0.0, 8.8, 13.7],b:[0.0, 8.8, 13.7],start:14.0,end:29.0},Grab{id:0,bone:Hash40::new("top").hash,radius:4.4,a:[0.0, 16.0, 6.5],b:[0.0, 18.0, 3.0],start:15.0,end:29.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("special_air_hi").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.4,a:[0.0, 16.0, 6.5],b:[0.0, 16.0, 6.5],start:14.0,end:29.0},Grab{id:1,bone:Hash40::new("top").hash,radius:6.5,a:[0.0, 8.8, 13.7],b:[0.0, 8.8, 13.7],start:14.0,end:29.0},Grab{id:0,bone:Hash40::new("top").hash,radius:4.4,a:[0.0, 16.0, 6.5],b:[0.0, 18.0, 3.0],start:15.0,end:29.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 10.0],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 11.8],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_GANON && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 9.0, -4.0],b:[0.0, 9.0, -15.6],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_GAOGAEN && motion==Hash40::new("special_s_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.0, 2.0],b:[0.0, 8.0, 8.0],start:18.0,end:35.0},Grab{id:1,bone:Hash40::new("top").hash,radius:1.5,a:[0.0, 8.0, 2.0],b:[0.0, 8.0, 10.5],start:18.0,end:35.0},];}
if kind==*FIGHTER_KIND_GAOGAEN && motion==Hash40::new("special_air_s_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.0, 2.0],b:[0.0, 8.0, 8.0],start:18.0,end:35.0},Grab{id:1,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 8.0, 2.0],b:[0.0, 8.0, 9.5],start:18.0,end:35.0},];}
if kind==*FIGHTER_KIND_GAOGAEN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 12.0],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_GAOGAEN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 11.8],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_GAOGAEN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 9.0, -4.0],b:[0.0, 9.0, -15.6],start:13.0,end:16.0},];}
if kind==*FIGHTER_KIND_GEKKOUGA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 16.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_GEKKOUGA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.0,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 17.5],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_GEKKOUGA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -25.3],start:14.0,end:16.0},];}
if kind==*FIGHTER_KIND_IKE && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 8.4],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_IKE && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 10.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_IKE && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -14.2],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_INKLING && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_INKLING && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.6],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_INKLING && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_JACK && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.1, 4.5],b:[0.0, 8.1, 9.0],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_JACK && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.6],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_JACK && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -7.0],b:[0.0, 6.6, -16.7],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_KAMUI && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 7.5, 4.0],b:[0.0, 7.5, 8.7],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_KAMUI && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 10.3],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_KAMUI && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -14.6],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_KEN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.1],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_KEN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 10.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_KEN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -14.1],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("special_n_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.0,a:[0.0, 6.0, 5.0],b:[0.0, 6.0, 5.0],start:18.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("special_n_loop").hash {return vec![Grab{id:1,bone:Hash40::new("top").hash,radius:5.8,a:[0.0, 6.4, 6.2],b:[0.0, 6.4, 6.2],start:0.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("special_air_n_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.0,a:[0.0, 6.0, 5.0],b:[0.0, 6.0, 5.0],start:18.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("special_air_n_loop").hash {return vec![Grab{id:1,bone:Hash40::new("top").hash,radius:5.8,a:[0.0, 6.4, 6.2],b:[0.0, 6.4, 6.2],start:0.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 9.4],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 11.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_KIRBY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 5.5, -4.0],b:[0.0, 5.5, -15.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_KOOPA && motion==Hash40::new("special_s_catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.5, 19.0],b:[0.0, 8.5, 19.0],start:6.0,end:7.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 8.0, 11.0],b:[0.0, 8.0, 11.0],start:6.0,end:7.0},];}
if kind==*FIGHTER_KIND_KOOPA && motion==Hash40::new("special_s_air_catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.5,a:[0.0, 6.0, 21.0],b:[0.0, 6.0, 21.0],start:6.0,end:7.0},Grab{id:1,bone:Hash40::new("top").hash,radius:5.5,a:[0.0, 7.0, 14.0],b:[0.0, 7.0, 14.0],start:6.0,end:7.0},];}
if kind==*FIGHTER_KIND_KOOPA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.0,a:[0.0, 8.0, 5.0],b:[0.0, 8.0, 14.0],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_KOOPA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.8,a:[0.0, 6.0, 2.0],b:[0.0, 6.0, 16.2],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_KOOPA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.0,a:[0.0, 8.0, -5.0],b:[0.0, 8.0, -22.0],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_KOOPAG && motion==Hash40::new("special_s_catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:10.0,a:[0.0, 10.5, 48.0],b:[0.0, 14.0, 26.0],start:38.0,end:39.0},];}
if kind==*FIGHTER_KIND_KOOPAG && motion==Hash40::new("special_s_air_catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:11.0,a:[0.0, 10.5, 48.0],b:[0.0, 19.0, 27.0],start:16.0,end:18.0},];}
if kind==*FIGHTER_KIND_KOOPAJR && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 6.6, 6.5],b:[0.0, 6.6, 14.0],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_KOOPAJR && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 6.6, 6.5],b:[0.0, 6.6, 15.0],start:14.0,end:17.0},];}
if kind==*FIGHTER_KIND_KOOPAJR && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 6.6, -6.5],b:[0.0, 6.6, -16.4],start:15.0,end:18.0},];}
if kind==*FIGHTER_KIND_KROOL && motion==Hash40::new("special_n_loop").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.0,a:[0.0, 8.0, 13.0],b:[0.0, 8.0, 18.0],start:0.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_KROOL && motion==Hash40::new("special_air_n_loop").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.0,a:[0.0, 8.0, 13.0],b:[0.0, 8.0, 18.0],start:0.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_KROOL && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.0,a:[0.0, 7.0, 5.0],b:[0.0, 7.0, 14.0],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_KROOL && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.8,a:[0.0, 7.0, 3.0],b:[0.0, 7.0, 10.6],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_KROOL && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.0,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -21.0],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_LINK && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 7.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_LINK && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 9.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_LINK && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -13.2],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_LITTLEMAC && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.9,a:[0.0, 8.0, 7.0],b:[0.0, 8.0, 10.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_LITTLEMAC && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.7],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_LITTLEMAC && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.9,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -13.6],start:14.0,end:16.0},];}
if kind==*FIGHTER_KIND_LUCARIO && motion==Hash40::new("special_s").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.0, 5.3],b:[0.0, 6.0, 1.0],start:9.0,end:10.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.0, 7.4],b:[0.0, 6.0, 1.0],start:9.0,end:10.0},];}
if kind==*FIGHTER_KIND_LUCARIO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 8.5, 4.0],b:[0.0, 8.5, 9.7],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_LUCARIO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 11.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_LUCARIO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -16.5],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_LUCAS && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 0.5],b:[0.0, 0.0, -5.0],start:12.0,end:18.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:18.0},];}
if kind==*FIGHTER_KIND_LUCAS && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 1.0],b:[0.0, 0.0, -5.0],start:14.0,end:20.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:16.0,end:20.0},];}
if kind==*FIGHTER_KIND_LUCAS && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 3.5],b:[0.0, 0.0, -5.0],start:15.0,end:21.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:17.0,end:21.0},];}
if kind==*FIGHTER_KIND_LUCINA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 7.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_LUCINA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.25, 4.0],b:[0.0, 7.25, 9.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_LUCINA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -13.2],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_LUIGI && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, -1.5],b:[0.0, 0.0, -1.5],start:14.0,end:21.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:14.0,end:21.0},];}
if kind==*FIGHTER_KIND_LUIGI && motion==Hash40::new("catch_l").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, -1.5],b:[0.0, 0.0, -1.5],start:14.0,end:21.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:14.0,end:21.0},];}
if kind==*FIGHTER_KIND_LUIGI && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, -1.5],b:[0.0, 0.0, -1.5],start:16.0,end:23.0},Grab{id:1,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:16.0,end:23.0},];}
if kind==*FIGHTER_KIND_LUIGI && motion==Hash40::new("catch_dash_l").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, -1.5],b:[0.0, 0.0, -1.5],start:16.0,end:23.0},Grab{id:1,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:16.0,end:23.0},];}
if kind==*FIGHTER_KIND_LUIGI && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 1.5],b:[0.0, 0.0, 1.5],start:17.0,end:24.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:17.0,end:24.0},];}
if kind==*FIGHTER_KIND_LUIGI && motion==Hash40::new("catch_turn_l").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 1.5],b:[0.0, 0.0, 1.5],start:17.0,end:24.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:17.0,end:24.0},];}
if kind==*FIGHTER_KIND_MARIO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MARIO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_MARIO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MARIOD && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MARIOD && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_MARIOD && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MARTH && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 7.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MARTH && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.25, 4.0],b:[0.0, 7.25, 9.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_MARTH && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -13.2],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MASTER && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.5],b:[0.0, 8.0, 9.0],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MASTER && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.6, 4.2],b:[0.0, 6.6, 9.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MASTER && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -5.8],b:[0.0, 6.6, -16.8],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_METAKNIGHT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.5, 4.0],b:[0.0, 6.5, 8.5],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_METAKNIGHT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.4,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 10.1],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_METAKNIGHT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -14.3],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("special_s").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.4,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},Grab{id:1,bone:Hash40::new("top").hash,radius:10.2,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("special_s_throw").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.4,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},Grab{id:1,bone:Hash40::new("top").hash,radius:10.2,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("special_air_s").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.4,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},Grab{id:1,bone:Hash40::new("top").hash,radius:10.2,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("special_air_s_throw").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.4,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},Grab{id:1,bone:Hash40::new("top").hash,radius:10.2,a:[0.0, 8.4, 17.0],b:[0.0, 8.4, 17.0],start:12.0,end:16.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.2,a:[0.0, 10.0, 4.5],b:[0.0, 10.0, 10.1],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, 4.5],b:[0.0, 8.5, 12.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MEWTWO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.2,a:[0.0, 8.5, -4.5],b:[0.0, 8.5, -17.3],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_MIIFIGHTER && motion==Hash40::new("special_s3_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 7.0, 5.0],b:[0.0, 8.0, 5.0],start:13.0,end:22.0},Grab{id:1,bone:Hash40::new("top").hash,radius:1.0,a:[0.0, 8.0, 8.0],b:[0.0, 8.0, 2.0],start:13.0,end:22.0},];}
if kind==*FIGHTER_KIND_MIIFIGHTER && motion==Hash40::new("special_air_s3_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 6.0, 3.5],b:[0.0, 6.0, 2.0],start:13.0,end:22.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, 5.5],b:[0.0, 8.0, 5.5],start:13.0,end:22.0},];}
if kind==*FIGHTER_KIND_MIIFIGHTER && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MIIFIGHTER && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_MIIFIGHTER && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MIIGUNNER && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 6.6, 7.3],b:[0.0, 6.6, 11.6],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MIIGUNNER && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.7, 4.6],b:[0.0, 6.7, 11.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_MIIGUNNER && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 6.4, -4.0],b:[0.0, 6.4, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MIISWORDSMAN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_MIISWORDSMAN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_MIISWORDSMAN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_MURABITO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 14.0],start:14.0,end:17.0},];}
if kind==*FIGHTER_KIND_MURABITO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 13.0],start:16.0,end:19.0},];}
if kind==*FIGHTER_KIND_MURABITO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 5.5, -5.0],b:[0.0, 5.5, -14.0],start:17.0,end:20.0},];}
if kind==*FIGHTER_KIND_NANA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 6.4],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_NANA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.9,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 8.1],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_NANA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 5.0, -4.0],b:[0.0, 5.0, -11.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_NESS && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 5.2, 4.0],b:[0.0, 5.2, 8.0],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_NESS && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.2,a:[0.0, 5.2, 4.0],b:[0.0, 5.2, 9.8],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_NESS && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 5.2, -4.0],b:[0.0, 5.2, -14.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_PACKUN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 11.0],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PACKUN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 11.5],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PACKUN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PACMAN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.2, 10.7],b:[0.0, 6.2, 10.7],start:12.0,end:34.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.5, 0.0],b:[0.0, -2.5, 0.0],start:12.0,end:34.0},Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.2, 10.7],b:[0.0, 6.2, 16.7],start:19.0,end:34.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:3.5,a:[0.0, 0.5, 0.0],b:[0.0, -2.5, 0.0],start:19.0,end:34.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.5, 0.0],b:[0.0, -2.5, 0.0],start:27.0,end:34.0},];}
if kind==*FIGHTER_KIND_PACMAN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.2, 10.7],b:[0.0, 6.2, 10.7],start:14.0,end:36.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.5, 0.5],b:[0.0, -2.5, 0.5],start:14.0,end:36.0},Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.2, 10.7],b:[0.0, 6.2, 16.7],start:22.0,end:36.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:3.5,a:[0.0, 0.5, 1.5],b:[0.0, -2.5, 1.5],start:22.0,end:36.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.5, 2.0],b:[0.0, -2.5, 2.0],start:30.0,end:36.0},];}
if kind==*FIGHTER_KIND_PACMAN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.2, -10.7],b:[0.0, 6.2, -10.7],start:15.0,end:37.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.5, 0.0],b:[0.0, -2.5, 0.0],start:15.0,end:37.0},Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 6.2, -10.7],b:[0.0, 6.2, -16.7],start:23.0,end:37.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:3.5,a:[0.0, 0.5, 0.0],b:[0.0, -2.5, 0.0],start:23.0,end:37.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.5, 0.0],b:[0.0, -2.5, 0.0],start:30.0,end:37.0},];}
if kind==*FIGHTER_KIND_PALUTENA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.4],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_PALUTENA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.9,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 13.1],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_PALUTENA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -18.9],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_PEACH && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, 4.0],b:[0.0, 8.5, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PEACH && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 8.5, 4.0],b:[0.0, 8.5, 10.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PEACH && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, -4.0],b:[0.0, 8.5, -14.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_PFUSHIGISOU && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 19.7],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_PFUSHIGISOU && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 19.7],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_PFUSHIGISOU && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 5.0, -4.0],b:[0.0, 5.0, -23.0],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_PICHU && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PICHU && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 10.6],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_PICHU && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PICKEL && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -4.0, -2.5],b:[0.0, -4.8, -2.5],start:13.0,end:28.0},Grab{id:2,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 7.0, 5.0],b:[0.0, 7.0, 9.0],start:13.0,end:28.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -5.0, -2.5],b:[0.0, -4.0, -2.5],start:22.0,end:28.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -3.0, -2.5],b:[0.0, -0.5, -2.5],start:24.0,end:28.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -3.0, -2.5],b:[0.0, 0.0, -2.5],start:26.0,end:28.0},];}
if kind==*FIGHTER_KIND_PICKEL && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -5.0, -2.5],b:[0.0, -4.8, -2.5],start:16.0,end:31.0},Grab{id:2,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 7.0, 5.0],b:[0.0, 7.0, 9.0],start:16.0,end:31.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -5.0, -2.5],b:[0.0, -4.0, -2.5],start:25.0,end:31.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -3.0, -2.5],b:[0.0, -0.5, -2.5],start:27.0,end:31.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -3.0, -2.5],b:[0.0, 0.0, -2.5],start:29.0,end:31.0},];}
if kind==*FIGHTER_KIND_PICKEL && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -5.0, 2.5],b:[0.0, -4.8, 2.5],start:17.0,end:32.0},Grab{id:2,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 7.0, -5.0],b:[0.0, 7.0, -9.0],start:17.0,end:32.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -5.0, 2.5],b:[0.0, -4.0, 2.5],start:26.0,end:32.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -3.0, 2.5],b:[0.0, -0.5, 2.5],start:28.0,end:32.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.5,a:[0.0, -3.0, 2.5],b:[0.0, 0.0, 2.5],start:30.0,end:32.0},];}
if kind==*FIGHTER_KIND_PIKACHU && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 5.9, 4.9],b:[0.0, 5.9, 7.9],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_PIKACHU && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 9.0],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_PIKACHU && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 5.0, -4.0],b:[0.0, 5.0, -12.7],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_PIT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.9],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PIT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 11.5],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PIT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -16.4],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_PITB && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.9],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PITB && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 11.5],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PITB && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -16.4],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_PLIZARDON && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.4,a:[0.0, 8.0, 7.5],b:[0.0, 8.0, 16.0],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_PLIZARDON && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.3,a:[0.0, 6.0, 5.0],b:[0.0, 6.0, 16.5],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_PLIZARDON && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.4,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -17.2],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_POPO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 6.4],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_POPO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.9,a:[0.0, 5.0, 4.0],b:[0.0, 5.0, 8.1],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_POPO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 5.0, -4.0],b:[0.0, 5.0, -11.4],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PURIN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 5.2, 4.0],b:[0.0, 5.2, 8.5],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PURIN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.8,a:[0.0, 5.2, 4.0],b:[0.0, 5.2, 10.2],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_PURIN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 5.2, -4.0],b:[0.0, 5.2, -14.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_PZENIGAME && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.0, 6.0],b:[0.0, 6.0, 10.7],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_PZENIGAME && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 10.6],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_PZENIGAME && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -14.7],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_REFLET && motion==Hash40::new("special_lw_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.0,a:[0.0, 10.0, 14.0],b:[0.0, 10.0, 14.0],start:20.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_REFLET && motion==Hash40::new("special_air_lw_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:7.0,a:[0.0, 10.0, 14.0],b:[0.0, 10.0, 14.0],start:20.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_REFLET && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.2],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_REFLET && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 11.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_REFLET && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -15.7],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_RICHTER && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_RICHTER && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 11.5],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_RICHTER && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:14.0,end:16.0},];}
if kind==*FIGHTER_KIND_RIDLEY && motion==Hash40::new("special_s_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.8,a:[0.0, 7.2, 16.0],b:[0.0, 7.2, 16.0],start:24.0,end:39.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 7.2, 6.5],b:[0.0, 7.2, 6.5],start:24.0,end:39.0},];}
if kind==*FIGHTER_KIND_RIDLEY && motion==Hash40::new("special_air_s_start").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:6.8,a:[0.0, 7.2, 16.0],b:[0.0, 7.2, 16.0],start:24.0,end:39.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 7.2, 6.5],b:[0.0, 7.2, 6.5],start:24.0,end:39.0},];}
if kind==*FIGHTER_KIND_RIDLEY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 7.5, 6.0],b:[0.0, 7.5, 11.5],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_RIDLEY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.5,a:[0.0, 7.5, 7.0],b:[0.0, 7.5, 11.5],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_RIDLEY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 7.5, -5.0],b:[0.0, 7.5, -18.0],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_ROBOT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.8,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 11.6],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_ROBOT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.2,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 12.2],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_ROBOT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.8,a:[0.0, 8.0, -2.0],b:[0.0, 8.0, -17.3],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_ROCKMAN && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.9],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_ROCKMAN && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.9,a:[0.0, 6.6, 4.5],b:[0.0, 6.6, 10.6],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_ROCKMAN && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.6,a:[0.0, 6.6, -3.8],b:[0.0, 6.6, -14.8],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_ROSETTA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 10.2],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_ROSETTA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 12.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_ROSETTA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -17.2],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_ROY && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 7.4],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_ROY && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.25, 4.0],b:[0.0, 7.25, 9.1],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_ROY && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -12.8],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_RYU && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 9.1],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_RYU && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 10.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_RYU && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -14.1],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_SAMUS && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:23.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.5, 13.0],b:[0.0, 7.5, 5.0],start:15.0,end:23.0},];}
if kind==*FIGHTER_KIND_SAMUS && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:17.0,end:25.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.5, 13.0],b:[0.0, 7.5, 5.0],start:17.0,end:25.0},];}
if kind==*FIGHTER_KIND_SAMUS && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:18.0,end:26.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.5, -18.0],b:[0.0, 7.5, -5.0],start:18.0,end:26.0},];}
if kind==*FIGHTER_KIND_SAMUSD && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:23.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.5, 13.0],b:[0.0, 7.5, 5.0],start:15.0,end:23.0},];}
if kind==*FIGHTER_KIND_SAMUSD && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:17.0,end:25.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.5, 13.0],b:[0.0, 7.5, 5.0],start:17.0,end:25.0},];}
if kind==*FIGHTER_KIND_SAMUSD && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:18.0,end:26.0},Grab{id:1,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.5, -18.0],b:[0.0, 7.5, -5.0],start:18.0,end:26.0},];}
if kind==*FIGHTER_KIND_SHEIK && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 9.4],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_SHEIK && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.5,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 9.5],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_SHEIK && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.1,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -13.4],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_SHIZUE && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 14.0],start:14.0,end:17.0},];}
if kind==*FIGHTER_KIND_SHIZUE && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 5.5, 4.0],b:[0.0, 5.5, 13.0],start:16.0,end:19.0},];}
if kind==*FIGHTER_KIND_SHIZUE && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:5.0,a:[0.0, 5.5, -5.0],b:[0.0, 5.5, -14.0],start:17.0,end:20.0},];}
if kind==*FIGHTER_KIND_SHULK && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 9.0],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_SHULK && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.8,a:[0.0, 8.0, 4.0],b:[0.0, 8.0, 10.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_SHULK && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.5,a:[0.0, 8.0, -4.0],b:[0.0, 8.0, -15.3],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_SIMON && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 8.7],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_SIMON && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.0],b:[0.0, 6.6, 11.5],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_SIMON && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.0],b:[0.0, 6.6, -14.7],start:14.0,end:16.0},];}
if kind==*FIGHTER_KIND_SNAKE && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.2, 7.0],b:[0.0, 8.2, 11.5],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_SNAKE && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.2, 7.0],b:[0.0, 8.2, 11.5],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_SNAKE && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:4.0,a:[0.0, 8.2, -11.0],b:[0.0, 8.2, -17.5],start:8.0,end:9.0},];}
if kind==*FIGHTER_KIND_SONIC && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 8.7],start:7.0,end:9.0},];}
if kind==*FIGHTER_KIND_SONIC && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.0, 4.0],b:[0.0, 6.0, 10.4],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_SONIC && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.0, -4.0],b:[0.0, 6.0, -14.7],start:11.0,end:13.0},];}
if kind==*FIGHTER_KIND_SZEROSUIT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:26.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:2.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:26.0},];}
if kind==*FIGHTER_KIND_SZEROSUIT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:17.0,end:27.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:2.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:17.0,end:27.0},];}
if kind==*FIGHTER_KIND_SZEROSUIT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:3.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:18.0,end:27.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:2.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:18.0,end:27.0},];}
if kind==*FIGHTER_KIND_TANTAN && motion==Hash40::new("catch_start").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, -1.0, -12.0],b:[0.0, -1.0, -0.5],start:1.0,end:51.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:1.4,a:[0.0, -1.0, -13.4],b:[0.0, -1.0, 0.9],start:1.0,end:51.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, -1.0, -1.5],b:[0.0, -1.0, -0.5],start:2.0,end:51.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:1.4,a:[0.0, -1.0, -2.9],b:[0.0, -1.0, 0.9],start:2.0,end:51.0},];}
if kind==*FIGHTER_KIND_TANTAN && motion==Hash40::new("catch_dash_start").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, -1.0, -12.0],b:[0.0, -1.0, -0.5],start:1.0,end:51.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:1.4,a:[0.0, -1.0, -13.4],b:[0.0, -1.0, 0.9],start:1.0,end:51.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, -1.0, -1.5],b:[0.0, -1.0, -0.5],start:2.0,end:51.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:1.4,a:[0.0, -1.0, -2.9],b:[0.0, -1.0, 0.9],start:2.0,end:51.0},];}
if kind==*FIGHTER_KIND_TANTAN && motion==Hash40::new("catch_turn_start").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, -1.0, -1.0],b:[0.0, -1.0, 7.0],start:1.0,end:51.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:1.4,a:[0.0, -1.0, -2.4],b:[0.0, -1.0, 8.4],start:1.0,end:51.0},Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, -1.0, 0.0],b:[0.0, -1.0, -1.0],start:2.0,end:51.0},Grab{id:1,bone:Hash40::new("throw").hash,radius:1.4,a:[0.0, -1.0, 1.4],b:[0.0, -1.0, -2.4],start:2.0,end:51.0},];}
if kind==*FIGHTER_KIND_TOONLINK && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:12.0,end:19.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:1.5,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:12.0,end:19.0},Grab{id:2,bone:Hash40::new("handr").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:12.0,end:19.0},];}
if kind==*FIGHTER_KIND_TOONLINK && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:21.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:1.5,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:21.0},Grab{id:2,bone:Hash40::new("handr").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:21.0},];}
if kind==*FIGHTER_KIND_TOONLINK && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:22.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:1.5,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:22.0},Grab{id:2,bone:Hash40::new("handr").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:22.0},];}
if kind==*FIGHTER_KIND_TRAIL && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, 4.6],b:[0.0, 6.6, 8.6],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_TRAIL && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 6.6, 4.6],b:[0.0, 6.6, 10.8],start:12.0,end:14.0},];}
if kind==*FIGHTER_KIND_TRAIL && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 6.6, -4.6],b:[0.0, 6.6, -11.6],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_WARIO && motion==Hash40::new("special_n_open_wait").hash {return vec![Grab{id:1,bone:Hash40::new("head").hash,radius:5.4,a:[-2.0, 2.0, 1.5],b:[-2.0, 2.0, 1.5],start:2.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_WARIO && motion==Hash40::new("special_air_n_open_wait").hash {return vec![Grab{id:1,bone:Hash40::new("head").hash,radius:5.4,a:[-2.0, 2.0, 1.5],b:[-2.0, 2.0, 1.5],start:2.0,end:1000000.0},];}
if kind==*FIGHTER_KIND_WARIO && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.7,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 10.3],start:8.0,end:11.0},];}
if kind==*FIGHTER_KIND_WARIO && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 12.0],start:11.0,end:14.0},];}
if kind==*FIGHTER_KIND_WARIO && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.7,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -15.9],start:12.0,end:15.0},];}
if kind==*FIGHTER_KIND_WIIFIT && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.6, 4.0],b:[0.0, 8.6, 8.2],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_WIIFIT && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.6,a:[0.0, 7.5, 4.0],b:[0.0, 7.5, 9.9],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_WIIFIT && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.3,a:[0.0, 8.5, -4.0],b:[0.0, 8.5, -14.0],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_WOLF && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 9.0],start:6.0,end:8.0},];}
if kind==*FIGHTER_KIND_WOLF && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:2.4,a:[0.0, 7.0, 4.0],b:[0.0, 7.0, 10.8],start:8.0,end:10.0},];}
if kind==*FIGHTER_KIND_WOLF && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 7.0, -4.0],b:[0.0, 7.0, -15.0],start:9.0,end:11.0},];}
if kind==*FIGHTER_KIND_YOSHI && motion==Hash40::new("special_n").hash {return vec![Grab{id:1,bone:Hash40::new("throw").hash,radius:3.5,a:[-2.0, -0.5, 0.0],b:[-2.0, -0.5, 0.0],start:17.0,end:21.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.9,a:[0.0, 7.0, 9.0],b:[0.0, 7.0, 9.0],start:17.0,end:21.0},Grab{id:1,bone:Hash40::new("mouth2").hash,radius:3.5,a:[-2.0, -0.5, 0.0],b:[-2.0, -0.5, 0.0],start:17.0,end:21.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.9,a:[0.0, 7.0, 9.0],b:[0.0, 7.0, 9.0],start:17.0,end:21.0},];}
if kind==*FIGHTER_KIND_YOSHI && motion==Hash40::new("special_air_n").hash {return vec![Grab{id:1,bone:Hash40::new("throw").hash,radius:3.5,a:[-2.0, -0.5, 0.0],b:[-2.0, -0.5, 0.0],start:17.0,end:21.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.9,a:[0.0, 7.0, 9.0],b:[0.0, 7.0, 9.0],start:17.0,end:21.0},Grab{id:1,bone:Hash40::new("mouth2").hash,radius:3.5,a:[-2.0, -0.5, 0.0],b:[-2.0, -0.5, 0.0],start:17.0,end:21.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.9,a:[0.0, 7.0, 9.0],b:[0.0, 7.0, 9.0],start:17.0,end:21.0},];}
if kind==*FIGHTER_KIND_YOSHI && motion==Hash40::new("catch").hash {return vec![Grab{id:1,bone:Hash40::new("mouth2").hash,radius:2.8,a:[1.2, -0.5, 0.0],b:[1.2, -0.5, 0.0],start:14.0,end:22.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.5,a:[0.0, 6.0, 7.5],b:[0.0, 6.0, 7.5],start:14.0,end:22.0},];}
if kind==*FIGHTER_KIND_YOSHI && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:1,bone:Hash40::new("mouth2").hash,radius:2.2,a:[1.2, -0.5, 0.0],b:[1.2, -0.5, 0.0],start:16.0,end:24.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.5,a:[0.0, 6.0, 9.0],b:[0.0, 6.0, 9.0],start:16.0,end:24.0},];}
if kind==*FIGHTER_KIND_YOSHI && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:1,bone:Hash40::new("mouth2").hash,radius:2.8,a:[1.2, -0.5, 0.0],b:[1.2, -0.5, 0.0],start:17.0,end:25.0},Grab{id:2,bone:Hash40::new("top").hash,radius:5.5,a:[0.0, 6.0, -8.0],b:[0.0, 6.0, -8.0],start:17.0,end:25.0},];}
if kind==*FIGHTER_KIND_YOUNGLINK && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:12.0,end:19.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:1.5,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:12.0,end:19.0},Grab{id:2,bone:Hash40::new("handr").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:12.0,end:19.0},];}
if kind==*FIGHTER_KIND_YOUNGLINK && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:21.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:1.5,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:21.0},Grab{id:2,bone:Hash40::new("handr").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:14.0,end:21.0},];}
if kind==*FIGHTER_KIND_YOUNGLINK && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("throw").hash,radius:2.8,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:22.0},Grab{id:1,bone:Hash40::new("handr").hash,radius:1.5,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:22.0},Grab{id:2,bone:Hash40::new("handr").hash,radius:4.0,a:[0.0, 0.0, 0.0],b:[0.0, 0.0, 0.0],start:15.0,end:22.0},];}
if kind==*FIGHTER_KIND_ZELDA && motion==Hash40::new("catch").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 11.5],start:10.0,end:12.0},];}
if kind==*FIGHTER_KIND_ZELDA && motion==Hash40::new("catch_dash").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.0,a:[0.0, 9.0, 4.0],b:[0.0, 9.0, 13.3],start:13.0,end:15.0},];}
if kind==*FIGHTER_KIND_ZELDA && motion==Hash40::new("catch_turn").hash {return vec![Grab{id:0,bone:Hash40::new("top").hash,radius:3.8,a:[0.0, 9.0, -4.0],b:[0.0, 9.0, -19.2],start:14.0,end:16.0},];}
vec![]
}
pub fn weapon_capsules(kind:i32)->u8 {
if kind==*WEAPON_KIND_BRAVE_DEATHBALL {return 3;}
if kind==*WEAPON_KIND_BRAVE_EXPLOSION {return 14;}
if kind==*WEAPON_KIND_BRAVE_LIGHTNING {return 1;}
if kind==*WEAPON_KIND_BRAVE_SLEEP {return 3;}
if kind==*WEAPON_KIND_BRAVE_SPARK {return 4;}
if kind==*WEAPON_KIND_BRAVE_TORNADO {return 64;}
if kind==*WEAPON_KIND_BUDDY_PAD {return 1;}
if kind==*WEAPON_KIND_CLOUD_WAVE {return 2;}
if kind==*WEAPON_KIND_DEDEDE_GORDO {return 7;}
if kind==*WEAPON_KIND_DEMON_BLASTERCHEST {return 31;}
if kind==*WEAPON_KIND_DEMON_BLASTERWING {return 63;}
if kind==*WEAPON_KIND_DIDDY_BUNSHIN {return 2;}
if kind==*WEAPON_KIND_DOLLY_WAVE {return 3;}
if kind==*WEAPON_KIND_DUCKHUNT_CLAY {return 6;}
if kind==*WEAPON_KIND_DUCKHUNT_GUNMANBULLET {return 31;}
if kind==*WEAPON_KIND_EDGE_FLARE1 {return 15;}
if kind==*WEAPON_KIND_EFLAME_BLAZEPILLAR {return 7;}
if kind==*WEAPON_KIND_EFLAME_ESWORD {return 112;}
if kind==*WEAPON_KIND_EFLAME_FIREPILLAR {return 1;}
if kind==*WEAPON_KIND_ELIGHT_BEAM {return 3;}
if kind==*WEAPON_KIND_ELIGHT_METEOR {return 1;}
if kind==*WEAPON_KIND_GANON_GANOND {return 6;}
if kind==*WEAPON_KIND_INKLING_MEGAPHONELASER {return 1;}
if kind==*WEAPON_KIND_INKLING_ROLLER {return 3;}
if kind==*WEAPON_KIND_KAMUI_DRAGONHAND {return 3;}
if kind==*WEAPON_KIND_KAMUI_SPEARHAND {return 13;}
if kind==*WEAPON_KIND_KEN_HADOKEN {return 3;}
if kind==*WEAPON_KIND_KEN_SHINRYUKEN {return 10;}
if kind==*WEAPON_KIND_KOOPAJR_BATTEN {return 3;}
if kind==*WEAPON_KIND_KOOPAJR_CANNONBALL {return 1;}
if kind==*WEAPON_KIND_KROOL_BACKPACK {return 1;}
if kind==*WEAPON_KIND_LINK_SWORD_BEAM {return 1;}
if kind==*WEAPON_KIND_LUCARIO_LUCARIOM {return 7;}
if kind==*WEAPON_KIND_LUCARIO_QIGONG {return 3;}
if kind==*WEAPON_KIND_LUCAS_PK_STARSTORM {return 1;}
if kind==*WEAPON_KIND_MARIO_HUGE_FLAME {return 15;}
if kind==*WEAPON_KIND_MARIOD_DRCAPSULE {return 1;}
if kind==*WEAPON_KIND_MARIOD_HUGE_CAPSULE {return 15;}
if kind==*WEAPON_KIND_MASTER_ARROW2 {return 7;}
if kind==*WEAPON_KIND_MIIGUNNER_ATTACKAIRF_BULLET {return 1;}
if kind==*WEAPON_KIND_MIIGUNNER_FLAMEPILLAR {return 2;}
if kind==*WEAPON_KIND_MIIGUNNER_FULLTHROTTLE {return 3;}
if kind==*WEAPON_KIND_MIIGUNNER_LASER {return 1;}
if kind==*WEAPON_KIND_MIIGUNNER_SUPERMISSILE {return 1;}
if kind==*WEAPON_KIND_MIISWORDSMAN_TORNADOSHOT {return 1;}
if kind==*WEAPON_KIND_POPO_ICEBERG {return 63;}
if kind==*WEAPON_KIND_POPO_ICEBERG_WIND {return 24;}
if kind==*WEAPON_KIND_POPO_ICEBERG_HIT {return 7;}
if kind==*WEAPON_KIND_NESS_PK_STARSTORM {return 1;}
if kind==*WEAPON_KIND_PACKUN_BOSSPACKUN {return 51;}
if kind==*WEAPON_KIND_PACMAN_TRAMPOLINE {return 1;}
if kind==*WEAPON_KIND_PALUTENA_BEAM {return 7;}
if kind==*WEAPON_KIND_PALUTENA_REFLECTIONBOARD {return 1;}
if kind==*WEAPON_KIND_PICHU_KAMINARI {return 1;}
if kind==*WEAPON_KIND_PICKEL_MELT {return 10;}
if kind==*WEAPON_KIND_PICKEL_MELT {return 3;}
if kind==*WEAPON_KIND_PICKEL_TROLLEY {return 6;}
if kind==*WEAPON_KIND_PIKACHU_KAMINARI {return 1;}
if kind==*WEAPON_KIND_PLIZARDON_DAIMONJI {return 2;}
if kind==*WEAPON_KIND_PLIZARDON_EXPLOSION {return 2;}
if kind==*WEAPON_KIND_POPO_ICEBERG {return 63;}
if kind==*WEAPON_KIND_POPO_ICEBERG_WIND {return 24;}
if kind==*WEAPON_KIND_POPO_ICEBERG_HIT {return 7;}
if kind==*WEAPON_KIND_PTRAINER_PFUSHIGISOU {return 3;}
if kind==*WEAPON_KIND_PTRAINER_PZENIGAME {return 15;}
if kind==*WEAPON_KIND_REFLET_CHROM {return 2;}
if kind==*WEAPON_KIND_REFLET_ELWIND {return 2;}
if kind==*WEAPON_KIND_REFLET_GIGAFIRE {return 2;}
if kind==*WEAPON_KIND_REFLET_THUNDER {return 6;}
if kind==*WEAPON_KIND_RICHTER_CROSS {return 3;}
if kind==*WEAPON_KIND_ROBOT_HUGEBEAM {return 1;}
if kind==*WEAPON_KIND_ROBOT_MAINLASER {return 1;}
if kind==*WEAPON_KIND_ROCKMAN_LEAFSHIELD {return 15;}
if kind==*WEAPON_KIND_ROSETTA_STARPIECE {return 1;}
if kind==*WEAPON_KIND_ROSETTA_TICO {return 4;}
if kind==*WEAPON_KIND_RYU_HADOKEN {return 14;}
if kind==*WEAPON_KIND_SAMUS_LASER {return 3;}
if kind==*WEAPON_KIND_SAMUS_LASER2 {return 1;}
if kind==*WEAPON_KIND_SAMUSD_LASER {return 1;}
if kind==*WEAPON_KIND_SAMUSD_LASER2 {return 1;}
if kind==*WEAPON_KIND_SHIZUE_TRAFFICSIGN {return 1;}
if kind==*WEAPON_KIND_SIMON_CROSS {return 3;}
if kind==*WEAPON_KIND_SNAKE_CYPHER {return 1;}
if kind==*WEAPON_KIND_SZEROSUIT_RETICLE {return 3;}
if kind==*WEAPON_KIND_TANTAN_BEAM {return 1;}
if kind==*WEAPON_KIND_TANTAN_RING {return 1;}
if kind==*WEAPON_KIND_TRAIL_HOLE {return 1;}
if kind==*WEAPON_KIND_WIIFIT_SILHOUETTE {return 1;}
if kind==*WEAPON_KIND_WIIFIT_SILHOUETTEL {return 7;}
if kind==*WEAPON_KIND_ZELDA_TRIFORCE {return 7;}
0
}