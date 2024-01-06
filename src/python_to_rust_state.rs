// takes a python object and makes it into a rlgym-sim-rs gamestate

use pyo3::{PyAny, types::PyList};
use rlgym_sim_rs::gamestates::{game_state::GameState, player_data::PlayerData, physics_object::{Quaternion, EulerAngle, RotationMatrix}};
use rocketsim_rs::sim::CarControls;

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
    // dbg!("hello");
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
    let mut state = GameState::new(Some(state_floats));
    for player_data in state.players.iter_mut(){
        player_data.car_data.euler_angles = euler_angles(player_data.car_data.quaternion);
        player_data.inverted_car_data.euler_angles = euler_angles(player_data.inverted_car_data.quaternion);
        player_data.car_data.has_computed_euler_angles = true;
        player_data.inverted_car_data.has_computed_euler_angles = true;

        player_data.car_data.rotation_mtx = rotation_mtx(player_data.car_data.quaternion);
        player_data.inverted_car_data.rotation_mtx = rotation_mtx(player_data.inverted_car_data.quaternion);
        player_data.car_data.has_computed_rot_mtx = true;
        player_data.inverted_car_data.has_computed_rot_mtx = true;
    }
    state
}

pub fn get_player(player: &PyAny) -> PlayerData{
    let mut player_floats = Vec::<f32>::with_capacity(PLAYER_INFO_LENGTH);
    player_floats.push(extract_attr!(player, car_id));
    player_floats.push(extract_attr!(player, team_num));
    player_floats.extend(get_car_physics_object_floats(extract_attr!(player, car_data)));
    player_floats.extend(get_car_physics_object_floats(extract_attr!(player, inverted_car_data)));
    player_floats.push(extract_attr!(player, match_goals));
    player_floats.push(extract_attr!(player, match_saves));
    player_floats.push(extract_attr!(player, match_shots));
    player_floats.push(extract_attr!(player, match_demolishes));
    player_floats.push(extract_attr!(player, boost_pickups));
    player_floats.push(extract_attr!(player, is_demoed));
    player_floats.push(extract_attr!(player, on_ground));
    player_floats.push(extract_attr!(player, ball_touched));
    player_floats.push(extract_attr!(player, has_jump));
    player_floats.push(extract_attr!(player, has_flip));
    player_floats.push(extract_attr!(player, boost_amount));
    decode_player_precompute(&player_floats)
}


fn get_ball_object_floats(obj: &PyAny) -> Vec<f32>{
    let mut floats = Vec::<f32>::with_capacity(9);
    floats.extend::<Vec<f32>>(extract_attr!(obj, position));
    floats.extend::<Vec<f32>>(extract_attr!(obj, linear_velocity));
    floats.extend::<Vec<f32>>(extract_attr!(obj, angular_velocity));
    floats
}

fn get_car_physics_object_floats(obj: &PyAny) -> Vec<f32>{
    let mut floats = Vec::<f32>::with_capacity(13);
    floats.extend::<Vec<f32>>(extract_attr!(obj, position));
    floats.extend::<Vec<f32>>(extract_attr!(obj, quaternion));
    floats.extend::<Vec<f32>>(extract_attr!(obj, linear_velocity));
    floats.extend::<Vec<f32>>(extract_attr!(obj, angular_velocity));
    floats
}

// copied from rlgym-sim-rs since it's private and this isn't Python
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

    player_data.car_data.euler_angles = euler_angles(player_data.car_data.quaternion);
    player_data.inverted_car_data.euler_angles = euler_angles(player_data.inverted_car_data.quaternion);
    player_data.car_data.has_computed_euler_angles = true;
    player_data.inverted_car_data.has_computed_euler_angles = true;

    player_data.car_data.rotation_mtx = rotation_mtx(player_data.car_data.quaternion);
    player_data.inverted_car_data.rotation_mtx = rotation_mtx(player_data.inverted_car_data.quaternion);
    player_data.car_data.has_computed_rot_mtx = true;
    player_data.inverted_car_data.has_computed_rot_mtx = true;

    // dbg!(player_data.car_data.euler_angles);
    // dbg!(player_data.car_data.rotation_mtx);

    player_data
}

pub fn get_car_controls_from_vec(action: &[f64]) -> CarControls{
    CarControls {
        throttle: action[0] as f32,
        steer: action[1] as f32,
        pitch: action[2] as f32,
        yaw: action[3] as f32,
        roll: action[4] as f32,
        jump: action[5] > 0.,
        boost: action[6] > 0.,
        handbrake: action[7] > 0.,
    }
}

pub fn euler_angles(quat: Quaternion) -> EulerAngle {
    // dbg!(quat);

    quat.quat_to_euler()
}

pub fn rotation_mtx(quat: Quaternion) -> RotationMatrix {
    quat.quat_to_rot_mtx()
}