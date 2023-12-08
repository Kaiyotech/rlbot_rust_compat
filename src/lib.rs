mod bytes; // replace this with rocketsim_rs with bin feature somehoow? Stuff is private though
mod gym;
mod utils;

use bytes::Vec3;
use ndarray::Dim;
use ndarray::Ix2;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::{PyReadonlyArray, PyArray, Ix1, IntoPyArray};
// use numpy::ndarray::{array, Array};

 use rlgym_sim_rs::{
     obs_builders::obs_builder::ObsBuilder,
     reward_functions::reward_fn::RewardFn,
 };

use crate::{
    gym::CompatGameState,
    utils::make_gym_state,
};

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
fn my_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pymodule!(reset, m)?)?;
    // m.add_function(wrap_pymodule!(pre_step, m)?)?;
    // m.add_function(wrap_pymodule!(build_obs, m)?)?;
    m.add_class::<CompatObs>()?;
    m.add_class::<CompatReward>()?;
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

// #[pyclass]
// struct CompatWrapper{
//     pub reward_fn: CompatReward,
//     pub obs_builder: CompatObs
// }

#[pyclass(unsendable)]
struct CompatReward{
    pub reward_fn: Box<dyn RewardFn>,
}

#[pyclass(unsendable)]
struct CompatObs{
    pub obs_builder: Box<dyn ObsBuilder>,
    previous_actions: Vec<Vec<f32>>,
}

#[pymethods]
impl CompatObs {
    #[new]
    pub fn new(obs_builder: dyn ObsBuilder) -> CompatObs {
        let previous_actions: Vec<Vec<f32>>;
        CompatObs{
            obs_builder,
            previous_actions
        }
    }
    pub fn reset(&mut self, state: CompatGameState){
        let gamestate = make_gym_state(state);
        self.obs_builder.reset(gamestate);
    }

    fn pre_step(&mut self, state: CompatGameState, previous_actions: PyReadonlyArray<f32, Ix2>){
        let gamestate = make_gym_state(state);
        for previous_action in previous_actions.iter(){
            gamestate.cars.last_action = previous_action;
        }
        self.previous_actions = previous_actions;
        self.obs_builder.pre_step(&gamestate);
    }

    fn build_obs<'py>(&mut self, py: Python<'py>, state: CompatGameState) -> PyResult<Py<PyArray<f32, Ix2>>>{
        let gamestate = make_gym_state(state);
        for previous_action in self.previous_actions.iter(){
            gamestate.cars.last_action = previous_action;
        }
        let obs: Vec<Vec<f32>>;
        for car in gamestate.cars.iter(){
            obs.push(self.obs_builder.build_obs(&car, &gamestate));
        }
    
        let obs_array = obs.into_pyarray(py);
        Ok(obs_array)
    }
}
