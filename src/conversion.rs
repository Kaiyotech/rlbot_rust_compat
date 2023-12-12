// takes a python object and makes it into a rlgym-sim-rs gamestate

use pyo3::PyAny;
use rlgym_sim_rs::gamestates::game_state::GameState;

pub fn get_state(obj: &PyAny) -> GameState{
    let mut state_floats = Some(vec![0.; 10]);
    GameState::new(state_floats)
}