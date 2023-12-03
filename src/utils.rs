use crate::{
    bytes::{BoostPad, BoostPadState, BallState, CarInfo, CarState, GameState, Team, Vec3},
    gym::GymState, BOOST_PADS_LENGTH,
};
use glam::{Mat3A, Quat};
use std::sync::RwLock;

#[inline]
pub fn array_to_quat(array: [f32; 4]) -> Quat {
    Quat::from_xyzw(-array[1], -array[2], array[3], array[0])
}

const TICK_RATE: f32 = 4. / 120.;
static BOOST_PAD_LOCATIONS: RwLock<[Vec3; BOOST_PADS_LENGTH]> = RwLock::new([Vec3::ZERO; BOOST_PADS_LENGTH]);
pub fn make_gym_state(gym_state: GymState) -> GameState {
    // construct the game state
    let game_state = GameState {
        tick_rate: TICK_RATE,
        tick_count: 0,
        ball: BallState {
            pos: gym_state.ball.position.into(),
            vel: gym_state.ball.linear_velocity.into(),
            ang_vel: gym_state.ball.angular_velocity.into(),
        },
        ball_rot: array_to_quat(gym_state.ball.quaternion),
        pads: BOOST_PAD_LOCATIONS
            .read()
            .unwrap()
            .into_iter()
            .zip(gym_state.boost_pads)
            .map(|(position, is_active)| BoostPad {
                position,
                is_big: position.z == 73.,
                state: BoostPadState {
                    is_active: is_active > 0.5,
                    ..Default::default()
                },
            })
            .collect(),
        cars: gym_state
            .players
            .into_iter()
            .enumerate()
            .map(|(id, player)| CarInfo {
                id: id as u32 + 1,
                team: if player.team_num < 0.5 { Team::Blue } else { Team::Orange },
                state: CarState {
                    pos: player.car_data.position.into(),
                    vel: player.car_data.linear_velocity.into(),
                    ang_vel: player.car_data.angular_velocity.into(),
                    rot_mat: Mat3A::from_quat(array_to_quat(player.car_data.quaternion)).into(),
                    is_on_ground: player.on_ground != 0,
                    is_demoed: player.is_demoed != 0,
                    has_flipped: player.has_flip == 0,
                    has_jumped: player.has_jump == 0,
                    ..Default::default()
                },
            })
            .collect(),
    };
    game_state
}