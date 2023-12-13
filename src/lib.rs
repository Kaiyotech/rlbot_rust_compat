mod python_to_rust_state;
mod rust_to_python_state;

use ndarray::{Ix2, Ix1, ArrayView2, Axis};
use pyo3::{prelude::*, pymodule, types::PyModule, PyResult, Python};
use numpy::{PyReadonlyArray, PyArray};

use rlgym_sim_rs::{AdvancedObs, reward_functions::{common_rewards::player_ball_rewards::VelocityPlayerToBallReward, reward_fn::RewardFn},
 envs::game_match::GameConfig, obs_builders::obs_builder::ObsBuilder};

pub use python_to_rust_state::{get_state, get_car_controls_from_vec, get_player};
pub use rust_to_python_state::set_state;

#[pymodule]
fn rlbot_rust_compat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CompatObs>()?;
    m.add_class::<CompatReward>()?;
    Ok(())
}


#[pyclass(unsendable)]
struct CompatObs{
    pub obs_builder: Box<dyn ObsBuilder>,
    pub gameconfig: GameConfig,
}

#[pymethods]
impl CompatObs {
    #[new]
    pub fn new(team_size: usize, tick_skip: usize, spawn_opponents: bool) -> CompatObs {
        // replace your game config here as necessary
        let gameconfig = GameConfig{ gravity: 1., boost_consumption: 1., team_size, tick_skip, spawn_opponents};
        // replace your obs builder here
        let obs_builder = Box::new(AdvancedObs::new());
        CompatObs{obs_builder, gameconfig}
    }
    pub fn reset(&mut self, py_state: &PyAny){
        let state = get_state(py_state);
        self.obs_builder.reset(&state);
    }

    fn pre_step(&mut self, py_state: &PyAny, previous_actions: PyReadonlyArray<f64, Ix2>){
        let mut state = get_state(py_state);
        let previous_actions_view: ArrayView2<f64> = previous_actions.as_array();
        for (i, row) in previous_actions_view.axis_iter(Axis(0)).enumerate() {
            state.players[i].last_actions = get_car_controls_from_vec(&row.to_vec());
        }
        self.obs_builder.pre_step(&state, &self.gameconfig);
    }

    fn build_obs(&mut self, py: Python<'_>, py_player: &PyAny, py_state: &PyAny, _previous_action: PyReadonlyArray<f64, Ix1>) -> PyResult<Py<PyArray<f32, Ix1>>>{
        let state = get_state(py_state);
        let player = get_player(py_player);
        let obs = self.obs_builder.build_obs(&player, &state, &self.gameconfig);
        let obs_array = PyArray::from_vec(py, obs).into();
        Ok(obs_array)
    }
}

#[pyclass(unsendable)]
struct CompatReward{
    pub reward_fn: Box<dyn RewardFn>,
}

#[pymethods]
impl CompatReward{
    #[new]
    pub fn new() -> CompatReward {
        // take any inputs necessary for your reward function
        // replace your reward function here
        let reward_fn = Box::new(VelocityPlayerToBallReward::new(Some(false)));
        CompatReward{reward_fn}
    }
    pub fn reset(&mut self, py_state: &PyAny){
        let state = get_state(py_state);
        self.reward_fn.reset(&state);
    }

    fn pre_step(&mut self, py_state: &PyAny, previous_actions: PyReadonlyArray<f64, Ix2>){
        let mut state = get_state(py_state);
        let previous_actions_view: ArrayView2<f64> = previous_actions.as_array();
        for (i, row) in previous_actions_view.axis_iter(Axis(0)).enumerate() {
            state.players[i].last_actions = get_car_controls_from_vec(&row.to_vec());
        }
        self.reward_fn.pre_step(&state);
    }

    fn get_reward(&mut self, py_player: &PyAny, py_state: &PyAny, _previous_action: PyReadonlyArray<f32, Ix1>) -> PyResult<f32>{
        let state = get_state(py_state);
        let player = get_player(py_player);
        let reward = self.reward_fn.get_reward(&player, &state);
        Ok(reward)
    }

    fn get_final_reward(&mut self, py_player: &PyAny, py_state: &PyAny, _previous_action: PyReadonlyArray<f32, Ix1>) -> PyResult<f32>{
        let state = get_state(py_state);
        let player = get_player(py_player);
        let reward = self.reward_fn.get_final_reward(&player, &state);
        Ok(reward)
    }
}


// example to test state to python
