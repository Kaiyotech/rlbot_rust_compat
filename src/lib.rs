mod conversion;

use ndarray::Ix2;
use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::PyReadonlyArray;

use rlgym_sim_rs::AdvancedObs;
use rlgym_sim_rs::reward_functions::common_rewards::player_ball_rewards::VelocityPlayerToBallReward;
use rlgym_sim_rs::envs::game_match::GameConfig;
use rlgym_sim_rs::{
     obs_builders::obs_builder::ObsBuilder,
     reward_functions::reward_fn::RewardFn,
 };

use conversion::get_state;


#[pymodule]
fn rlbot_rust_compat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CompatObs>()?;
    m.add_class::<CompatReward>()?;
    Ok(())
}

#[pyclass(unsendable)]
struct CompatReward{
    pub reward_fn: Box<dyn RewardFn>,
}

#[pyclass(unsendable)]
struct CompatObs{
    pub obs_builder: Box<dyn ObsBuilder>,
}

#[pymethods]
impl CompatObs {
    #[new]
    pub fn new() -> CompatObs {
        // replace your game config here as necessary
        let obs_builder = Box::new(AdvancedObs::new());
        CompatObs{obs_builder}
    }
    pub fn reset(&mut self, py_state: &PyAny){
        let state = get_state(py_state);
        self.obs_builder.reset(&state);
    }

    fn pre_step(&mut self, py_state: &PyAny, previous_actions: PyReadonlyArray<f32, Ix2>){
        let state = get_state(py_state);
        for (i, previous_action) in previous_actions.to_vec().unwrap().iter().enumerate(){
            state.players[i].last_actions = get_car_controls_from_vec(&vec![*previous_action]);
        }
        self.obs_builder.pre_step(&gamestate, &self.game_config);
    }

    fn build_obs<'py>(&mut self, py: Python<'py>, player: CompatPlayerData, state: CompatGameState, _previous_action: PyReadonlyArray<f32, Ix1>) -> PyResult<Py<PyArray<f32, Ix1>>>{
        let gamestate = make_sim_state(state);
        let id: usize = player.car_id as usize;
        let state = self.simulator.get_rlgym_gamestate(false).0;
        let obs = self.obs_builder.build_obs(&state.players[id - 1], &state, &self.game_config);
    
        // let obs_array = obs.into_pyarray(py);
        let obs_array = PyArray::from_vec(py, obs).into();
        Ok(obs_array)
    }
}
