// This all comes from gym.rs in rlgym-sim-rs::gamestates but has to be copied here to wrap it in derive (I think anyway)

use pyo3::prelude::*;
use std::ffi::c_uchar;
use rlgym_sim_rs::common_values::BOOST_LOCATIONS;
// use rlgym_sim_rs::gamestates::{game_state::GameState,
//     player_data::PlayerData,
//     physics_object::{EulerAngle,
//         PhysicsObject,
//         Position,
//         IterCounterPos,
//         IterCounterVel,
//         Quaternion,
//         RotationMatrix,
//     }
// };

pub const BOOST_PADS_LENGTH: usize = BOOST_LOCATIONS.len();

#[derive(FromPyObject, Debug)]
pub struct CompatPhysicsObject {
    pub position: [f32; 3],
    pub quaternion: [f32; 4],
    pub linear_velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
}

#[derive(FromPyObject, Debug)]
pub struct CompatPlayerData {
    pub car_id: u32,
    pub team_num: f32,
    pub match_goals: f32,
    pub match_saves: f32,
    pub match_shots: f32,
    pub match_demolishes: f32,
    pub boost_pickups: f32,
    pub is_demoed: c_uchar,
    pub on_ground: c_uchar,
    pub ball_touched: c_uchar,
    pub has_jump: c_uchar,
    pub has_flip: c_uchar,
    pub boost_amount: f32,
    pub car_data: CompatPhysicsObject,
}

#[derive(FromPyObject, Debug)]
pub struct CompatGameState {
    // pub game_type: f32,
    // pub blue_score: f32,
    // pub orange_score: f32,
    // pub last_touch: f32,
    pub ball: CompatPhysicsObject,
    pub boost_pads: [f32; BOOST_PADS_LENGTH],
    pub players: Vec<CompatPlayerData>,
}


// can't do this because of trait bound issues with pyclass
// #[derive(FromPyObject, Debug, Clone)]
// pub struct CompatGameState(GameState);

// #[derive(FromPyObject, Debug, Clone)]
// pub struct CompatPlayerData(PlayerData);

// #[derive(FromPyObject, Debug, Clone)]
// pub struct CompatPhysics(PhysicsObject);