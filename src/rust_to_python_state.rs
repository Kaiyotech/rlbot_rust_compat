// takes a rlgym-sim-rs gamestate and makes it into a state array which can be handled in Python
use rlgym_sim_rs::gamestates::{game_state::GameState, physics_object::PhysicsObject};

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


pub fn set_state(state: GameState) -> Vec<f32>{
    let mut state_floats = Vec::<f32>::with_capacity(MAX_SIZE);
    state_floats.push(0.); // game type but not used
    state_floats.push(state.blue_score as f32);
    state_floats.push(state.orange_score as f32);
    state_floats.extend(state.boost_pads);
    state_floats.extend(get_ball_object_floats(&state.ball));
    state_floats.extend(get_ball_object_floats(&state.inverted_ball));

    for player in state.players.iter(){
        state_floats.push(player.car_id as f32);
        state_floats.push(player.team_num as f32);
        state_floats.extend(get_car_physics_object_floats(&player.car_data));
        state_floats.extend(get_car_physics_object_floats(&player.inverted_car_data));
        state_floats.push(player.match_goals as f32);
        state_floats.push(player.match_saves as f32);
        state_floats.push(player.match_shots as f32);
        state_floats.push(player.match_demolishes as f32);
        state_floats.push(player.boost_pickups as f32);
        state_floats.push(player.is_demoed as u8 as f32);
        state_floats.push(player.on_ground as u8 as f32);
        state_floats.push(player.ball_touched as u8 as f32);
        state_floats.push(player.has_jump as u8 as f32);
        state_floats.push(player.has_flip as u8 as f32);
        state_floats.push(player.boost_amount);
    }
    state_floats
}

fn get_ball_object_floats(ball: &PhysicsObject) -> Vec<f32>{
    let mut floats = Vec::<f32>::with_capacity(9);
    floats.extend(ball.position);
    floats.extend(ball.linear_velocity);
    floats.extend(ball.angular_velocity);
    floats
}

fn get_car_physics_object_floats(data: &PhysicsObject) -> Vec<f32>{
    let mut floats = Vec::<f32>::with_capacity(13);
    floats.extend(data.position);
    floats.extend(data.quaternion.into_array());
    floats.extend(data.linear_velocity);
    floats.extend(data.angular_velocity);
    floats
}