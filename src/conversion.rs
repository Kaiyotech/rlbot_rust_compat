// takes a python object and makes it into a rlgym-sim-rs gamestate

use std::f32::consts::E;

use pyo3::{PyAny, types::PyList};
use rlgym_sim_rs::gamestates::game_state::GameState;

//blue_score = 1
//orange_score = 2
//boost = 3 through length + 3 (34 length)
//ball = ball length and inverted ball both fit in ball length (ball length 18)
//num_players * player length per (39)
const BOOST_PAD_LENGTH: usize = 34;
const BALL_STATE_LENGTH: usize = 18;
const PLAYER_CAR_STATE_LENGTH: usize = 13;
const PLAYER_TERTIARY_INFO_LENGTH: usize = 11;
const PLAYER_INFO_LENGTH: usize = 2 + 2 * PLAYER_CAR_STATE_LENGTH + PLAYER_TERTIARY_INFO_LENGTH;
const MAX_SIZE: usize = 3 + BOOST_PAD_LENGTH + BALL_STATE_LENGTH + 6 * PLAYER_INFO_LENGTH;

// 
macro_rules! extract_attr {
    ($obj:expr, $attr:ident) => {
        $obj.getattr(stringify!($attr)).unwrap().extract().unwrap()
    };
}

pub fn get_state(obj: &PyAny) -> GameState{
    let mut state_floats = Vec::<f32>::with_capacity(MAX_SIZE);
    // state_floats.push(obj.getattr("blue_score").unwrap().extract().unwrap());
    state_floats.push(0.); // game type but not used
    state_floats.push(extract_attr!(obj, blue_score));
    state_floats.push(extract_attr!(obj, orange_score));
    state_floats.extend::<Vec<f32>>(extract_attr!(obj, boost_pads));
    state_floats.extend(get_ball_object_floats(extract_attr!(obj, ball)));
    state_floats.extend(get_ball_object_floats(extract_attr!(obj, inverted_ball)));

    // let players = obj.getattr("players").unwrap().extract::<PyList>().unwrap();//.extract().unwrap();//.len();
    let players = obj.getattr("players").unwrap().downcast::<PyList>().unwrap();
    for player in players.iter(){
        state_floats.push(extract_attr!(player, car_id));
        state_floats.push(extract_attr!(player, team_num));
        state_floats.extend(get_car_physics_object_floats(extract_attr!(player, car_data)));
        state_floats.extend(get_car_physics_object_floats(extract_attr!(player, inverted_car_data)));
        state_floats.push(extract_attr!(player, match_goals));
        state_floats.push(extract_attr!(player, match_saves));
        state_floats.push(extract_attr!(player, match_shots));
        state_floats.push(extract_attr!(player, match_demolishes));
        state_floats.push(extract_attr!(player, boost_pickups));
        state_floats.push(extract_attr!(player, is_demoed));
        state_floats.push(extract_attr!(player, on_ground));
        state_floats.push(extract_attr!(player, ball_touched));
        state_floats.push(extract_attr!(player, has_jump));
        state_floats.push(extract_attr!(player, has_flip));
        state_floats.push(extract_attr!(player, boost_amount));
    }
    // dbg!(state_floats.clone());
    GameState::new(Some(state_floats))
}


fn get_ball_object_floats(obj: &PyAny) -> Vec<f32>{
    let mut floats = Vec::<f32>::with_capacity(9);
    floats.extend::<Vec<f32>>(extract_attr!(obj, position));
    floats.extend::<Vec<f32>>(extract_attr!(obj, linear_velocity));
    floats.extend::<Vec<f32>>(extract_attr!(obj, angular_velocity));
    dbg!(floats.clone());
    floats
}

fn get_car_physics_object_floats(obj: &PyAny) -> Vec<f32>{
    let mut floats = Vec::<f32>::with_capacity(13);
    floats.extend::<Vec<f32>>(extract_attr!(obj, position));
    floats.extend::<Vec<f32>>(extract_attr!(obj, quaternion));
    floats.extend::<Vec<f32>>(extract_attr!(obj, linear_velocity));
    floats.extend::<Vec<f32>>(extract_attr!(obj, angular_velocity));
    dbg!(floats.clone());
    floats
}