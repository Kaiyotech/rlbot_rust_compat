mod bytes; // replace this with rocketsim_rs with bin feature
mod gym;
mod utils;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::{PyReadonlyArray, PyArray, Ix1, IntoPyArray};
// use numpy::ndarray::{array, Array};

use crate::{
    gym::GymState,
    utils::make_gym_state,
};
use bytes::{BallState, CarInfo, CarState, GameState, Team, Vec3};
use gym::BOOST_PADS_LENGTH;

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
    m.add_class::<ObsBuilder>()?;
    Ok(())
}

#[pyclass]
struct ObsBuilder {
    #[pyo3(get, set)]
    n: usize,
    kickoff_timer: isize,
    tick_skip: isize,
    infinite_boost_odds: f32,
}

#[pymethods]
impl ObsBuilder {
    #[new]
    pub fn new(tick_skip: isize, infinite_boost_odds: f32) -> ObsBuilder {
        ObsBuilder{n: 0,
            kickoff_timer: 0,
            tick_skip,
            infinite_boost_odds}
    }
    pub fn reset(&mut self, state: GymState){
        let gamestate = make_gym_state(state);
        self.n = 0;
        self.kickoff_timer = 0;

    }

    fn pre_step(&mut self, state: GymState){
        let gamestate = make_gym_state(state);
        self.n = 0;
        self.kickoff_timer += 1;
    }

    fn build_obs<'py>(&mut self, py: Python<'py>, state: GymState, previous_action: PyReadonlyArray<f64, Ix1>) -> PyResult<&'py PyArray<f32, Ix1>>{
        let gamestate = make_gym_state(state);
        // println!("{:#?}", gamestate);
        let player = &gamestate.cars[self.n];
        // println!("{:#?}", player);
        let obs = vec![
            self.kickoff_timer as f32,
            self.n as f32,
            player.id as f32,
            player.state.vel.x / VEL_STD,
            player.state.vel.y / VEL_STD,
            player.state.vel.z / VEL_STD,
            player.state.pos.x / POS_STD,
            player.state.pos.y / POS_STD,
            player.state.pos.z / POS_STD,
            player.state.ang_vel.x / ANG_STD,
            player.state.ang_vel.y / ANG_STD,
            player.state.ang_vel.z / ANG_STD
        ];

        self.n += 1;
    
        let obs_array = obs.into_pyarray(py);
        Ok(obs_array)
    }
}
