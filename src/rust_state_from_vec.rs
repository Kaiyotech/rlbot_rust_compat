// use core::num;

// use rayon::prelude::*;

use rocketsim_rs::sim::{BallHitInfo, CarControls};
// use serde::{Serialize, Deserialize};

use rlgym_sim_rs::common_values::BLUE_TEAM;
use rlgym_sim_rs::gamestates::player_data::PlayerData;

use rlgym_sim_rs::gamestates::{game_state::GameState, physics_object::{Position, Velocity,PhysicsObject}};
use rocketsim_rs::BoostPad;


const BOOST_PAD_LENGTH: usize = 34;
const BALL_STATE_LENGTH: usize = 18;
const PLAYER_CAR_STATE_LENGTH: usize = 13;
const PLAYER_TERTIARY_INFO_LENGTH: usize = 11;
const PLAYER_INFO_LENGTH: usize = 2 + 2 * PLAYER_CAR_STATE_LENGTH + PLAYER_TERTIARY_INFO_LENGTH;

pub trait StateFromVec {
    fn new_from_vec(state_floats: Vec<f32>) -> GameState;
    fn decode(&mut self, state_floats: Vec<f32>);
}


impl StateFromVec for GameState{
    fn new_from_vec(state_floats: Vec<f32>) -> GameState {
        let mut game_st = GameState {
            game_type: 0,
            blue_score: -1,
            orange_score: -1,
            last_touch: -1,
            players: Vec::new(),
            ball: PhysicsObject::new(),
            inverted_ball: PhysicsObject::new(),
            boost_pads: [BoostPad::default(); 34],
            inverted_boost_pads: [BoostPad::default(); 34],
            tick_num: 0,
        };
        game_st.decode(state_floats);
        game_st
    }
        // Default::default()
        // GameState {
        //     game_type: 0,
        //     blue_score: -1,
        //     orange_score: -1,
        //     last_touch: -1,
        //     players: Vec::<PlayerData>::new(),
        //     ball: PhysicsObject::new(),
        //     inverted_ball: PhysicsObject::new(),
        //     boost_pads: Vec::<f32>::new(),
        //     inverted_boost_pads: Vec::<f32>::new()
        // }
    

    fn decode(&mut self, state_vals: Vec<f32>) {
        let mut start = 3;
        let num_ball_packets = 1;
        let state_val_len = state_vals.len();

        let num_player_packets =
            ((state_val_len as i32 - num_ball_packets * BALL_STATE_LENGTH as i32 - start as i32 - BOOST_PAD_LENGTH as i32) / PLAYER_INFO_LENGTH as i32) as usize;

        self.blue_score = state_vals[1] as i32;
        self.orange_score = state_vals[2] as i32;

        // self.boost_pads.iter_mut().map(|pad| state_vals[start..start + BOOST_PAD_LENGTH].try_into().unwrap();
        for (i, pad_value) in state_vals[start..start + BOOST_PAD_LENGTH].iter().enumerate(){
            self.boost_pads[i].state.is_active = *pad_value > 0.;
        }
        self.inverted_boost_pads = self.boost_pads;
        self.inverted_boost_pads.reverse();
        start += BOOST_PAD_LENGTH;

        self.ball.decode_ball_data(&state_vals[start..start + BALL_STATE_LENGTH]);
        start += BALL_STATE_LENGTH / 2;

        self.inverted_ball.decode_ball_data(&state_vals[start..start + BALL_STATE_LENGTH]);
        start += BALL_STATE_LENGTH / 2;

        self.players.reserve(num_player_packets);

        self.players = ((start..start + (PLAYER_INFO_LENGTH * num_player_packets))
            .step_by(PLAYER_INFO_LENGTH))
            .map(|start| decode_player_precompute(&state_vals[start..start + PLAYER_INFO_LENGTH]))
            .collect::<Vec<PlayerData>>();

        self.players.sort_unstable_by_key(|p| p.car_id);
    }

    // fn decode_player(&self, full_player_data: &[f64]) -> PlayerData {
    //     let mut player_data = PlayerData::new();

    //     let mut start: usize = 2;

    //     player_data.car_data.decode_car_data(&full_player_data[start..start+PLAYER_CAR_STATE_LENGTH]);
    //     start = start + PLAYER_CAR_STATE_LENGTH;

    //     player_data.inverted_car_data.decode_car_data(&full_player_data[start..start+PLAYER_CAR_STATE_LENGTH]);
    //     start = start + PLAYER_CAR_STATE_LENGTH;

    //     let tertiary_data = &full_player_data[start..start+PLAYER_TERTIARY_INFO_LENGTH];

    //     player_data.match_goals = tertiary_data[0] as i64;
    //     player_data.match_saves = tertiary_data[1] as i64;
    //     player_data.match_shots = tertiary_data[2] as i64;
    //     player_data.match_demolishes = tertiary_data[3] as i64;
    //     player_data.boost_pickups = tertiary_data[4] as i64;
    //     player_data.is_demoed = tertiary_data[5] > 0.;
    //     player_data.on_ground = tertiary_data[6] > 0.;
    //     player_data.ball_touched = tertiary_data[7] > 0.;
    //     player_data.has_jump = tertiary_data[8] > 0.;
    //     player_data.has_flip = tertiary_data[9] > 0.;
    //     player_data.boost_amount = tertiary_data[10];
    //     player_data.car_id = full_player_data[0] as i32;
    //     player_data.team_num = full_player_data[1] as i32;

    //     return player_data
    // }

}

fn decode_player_precompute(full_player_data: &[f32]) -> PlayerData {
    let mut player_data = PlayerData::new();

    let mut start: usize = 2;

    player_data.car_data.decode_car_data(&full_player_data[start..start + PLAYER_CAR_STATE_LENGTH]);
    start += PLAYER_CAR_STATE_LENGTH;

    player_data.inverted_car_data.decode_car_data(&full_player_data[start..start + PLAYER_CAR_STATE_LENGTH]);
    start += PLAYER_CAR_STATE_LENGTH;

    let tertiary_data = &full_player_data[start..start + PLAYER_TERTIARY_INFO_LENGTH];

    player_data.match_goals = tertiary_data[0] as i64;
    player_data.match_saves = tertiary_data[1] as i64;
    player_data.match_shots = tertiary_data[2] as i64;
    player_data.match_demolishes = tertiary_data[3] as i64;
    player_data.boost_pickups = tertiary_data[4] as i64;
    player_data.is_demoed = tertiary_data[5] > 0.;
    player_data.on_ground = tertiary_data[6] > 0.;
    player_data.ball_touched = tertiary_data[7] > 0.;
    player_data.has_jump = tertiary_data[8] > 0.;
    player_data.has_flip = tertiary_data[9] > 0.;
    player_data.boost_amount = tertiary_data[10];
    player_data.car_id = full_player_data[0] as i32;
    player_data.team_num = full_player_data[1] as i32;

    // player_data.car_data.euler_angles();
    // player_data.inverted_car_data.euler_angles();

    player_data.car_data.rotation_mtx();
    player_data.inverted_car_data.rotation_mtx();

    player_data
}
