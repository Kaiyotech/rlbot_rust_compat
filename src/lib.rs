// mod bytes; // replace this with rocketsim_rs with bin feature somehoow? Stuff is private though
mod gym;
// mod sim_state;
mod gym_state;
mod conversion;

// use bytes::Vec3;
use gym_state::get_car_controls_from_vec;
use ndarray::Dim;
use ndarray::Ix2;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::{PyReadonlyArray, PyArray, Ix1, IntoPyArray};
// use numpy::ndarray::{array, Array};

 use rlgym_sim_rs::envs::game_match::GameConfig;
use rlgym_sim_rs::obs_builders::obs_builder;
use rlgym_sim_rs::{
     obs_builders::obs_builder::ObsBuilder,
     reward_functions::reward_fn::RewardFn,
 };

use crate::{
    gym::CompatGameState,
    gym::CompatPlayerData,
    // sim_state::make_sim_state,
    gym_state::RocketsimWrapper,
};

use conversion::get_state;

// use rocketsim_rs::bytes;
use crate::gym::BOOST_PADS_LENGTH;

use std::sync::RwLock;

const TICK_RATE: f32 = 4. / 120.;
static BOOST_PAD_LOCATIONS: RwLock<[Vec3; BOOST_PADS_LENGTH]> = RwLock::new([Vec3::ZERO; BOOST_PADS_LENGTH]);
const POS_STD: f32 = 2300.0;
const VEL_STD: f32 = 2300.0;
const ANG_STD: f32 = 5.5;
const DODGE_DEADZONE: f32 = 0.5;
const BOOST_TIMER_STD: f32 = 5.5;
const DEMO_TIMER_STD: f32 = 3.;

#[pymodule]
fn rlbot_rust_compat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CompatObs>()?;
    m.add_class::<CompatReward>()?;
    // m.add_class::<CompatWrapper>()?;
    Ok(())
}

// #[pyclass]
// struct CompatObsBuilder {
//     #[pyo3(get, set)]
//     n: usize,
//     kickoff_timer: isize,
//     tick_skip: isize,
//     infinite_boost_odds: f32,
// }

// #[pyclass[unsendable]]
// struct CompatWrapper{
//     pub reward_fn: CompatReward,
//     pub obs_builder: CompatObs,
//     pub sim_wrapper: RocketsimWrapper,
// }

#[pyclass(unsendable)]
struct CompatReward{
    pub reward_fn: Box<dyn RewardFn>,
}

#[pyclass(unsendable)]
struct CompatObs{
    pub obs_builder: Box<dyn ObsBuilder>,
    previous_actions: Vec<Vec<f32>>,
    simulator: RocketsimWrapper,
    game_config: GameConfig,
}

// #[pymethods]
// impl CompatWrapper{
//     #[new]
//     pub fn new(obs_builder: Box<dyn ObsBuilder>, reward_fn: Box<dyn RewardFn>, sim_wrapper: Box<RocketsimWrapper>) -> CompatWrapper{
//         CompatWrapper { reward_fn, obs_builder, sim_wrapper }
//     }
// }

#[pymethods]
impl CompatObs {
    #[new]
    pub fn new(tick_skip: usize, spawn_opponents: bool, team_size: usize) -> CompatObs {
        let game_config = GameConfig {
            tick_skip,
            spawn_opponents,
            team_size,
            gravity: 1.,
            boost_consumption: 1.,
        };
        let simulator = RocketsimWrapper::new(game_config);
        let previous_actions: Vec<Vec<f32>>;
        let obs_builder = Box::<AdvancedObs::new()>;
        CompatObs{
            obs_builder,
            previous_actions,
            simulator,
            game_config,
        }
    }
    pub fn reset(&mut self, py_state: &PyAny){
        // let gamestate = make_sim_state(py_state);
        // let state = self.simulator.get_rlgym_gamestate(false).0;
        let state = get_state(py_state);
        self.obs_builder.reset(&state);
    }

//     fn pre_step(&mut self, py_state: CompatGameState, previous_actions: PyReadonlyArray<f32, Ix2>){
//         let gamestate = make_sim_state(py_state);
//         let state = self.simulator.get_rlgym_gamestate(false).0;
//         for (i, previous_action) in previous_actions.to_vec().unwrap().iter().enumerate(){
//             state.players[i].last_actions = get_car_controls_from_vec(&vec![*previous_action]);
//         }
//         self.obs_builder.pre_step(&gamestate, &self.game_config);
//     }

//     fn build_obs<'py>(&mut self, py: Python<'py>, player: CompatPlayerData, state: CompatGameState, _previous_action: PyReadonlyArray<f32, Ix1>) -> PyResult<Py<PyArray<f32, Ix1>>>{
//         let gamestate = make_sim_state(state);
//         let id: usize = player.car_id as usize;
//         let state = self.simulator.get_rlgym_gamestate(false).0;
//         let obs = self.obs_builder.build_obs(&state.players[id - 1], &state, &self.game_config);
    
//         // let obs_array = obs.into_pyarray(py);
//         let obs_array = PyArray::from_vec(py, obs).into();
//         Ok(obs_array)
//     }
}
