// use ndarray::Ix2;
// use pyo3::prelude::*;
// use pyo3::{pymodule, types::PyModule, PyResult, Python};
// use numpy::{PyReadonlyArray, PyArray, Ix1};

//  use rlgym_sim_rs::envs::game_match::GameConfig;
// use rlgym_sim_rs::{
//      obs_builders::obs_builder::ObsBuilder,
//      obs_builders::advanced_obs::AdvancedObs,
//      reward_functions::reward_fn::RewardFn,
//  };

// use rlbot_rust_compat::{CompatGameState,
//     CompatPlayerData,
//     BOOST_PADS_LENGTH,
//     make_sim_state,
//     RocketsimWrapper,
//     get_car_controls_from_vec,
//     Vec3
// };

// use std::sync::RwLock;

// const TICK_RATE: f32 = 4. / 120.;
// static BOOST_PAD_LOCATIONS: RwLock<[Vec3; BOOST_PADS_LENGTH]> = RwLock::new([Vec3::ZERO; BOOST_PADS_LENGTH]);
// const POS_STD: f32 = 2300.0;
// const VEL_STD: f32 = 2300.0;
// const ANG_STD: f32 = 5.5;
// const DODGE_DEADZONE: f32 = 0.5;
// const BOOST_TIMER_STD: f32 = 5.5;
// const DEMO_TIMER_STD: f32 = 3.;

// #[pymodule]
// fn my_rust_compat(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<CompatObs>()?;
//     m.add_class::<CompatReward>()?;
//     // m.add_class::<CompatWrapper>()?;
//     Ok(())
// }

// #[pyclass(unsendable)]
// struct CompatReward{
//     pub reward_fn: Box<dyn RewardFn>,
// }

// #[pyclass(unsendable)]
// struct CompatObs{
//     pub obs_builder: Box<dyn ObsBuilder>,
//     previous_actions: Vec<Vec<f32>>,
//     simulator: RocketsimWrapper,
//     game_config: GameConfig,
// }

// // #[pymethods]
// // impl CompatWrapper{
// //     #[new]
// //     pub fn new(obs_builder: Box<dyn ObsBuilder>, reward_fn: Box<dyn RewardFn>, sim_wrapper: Box<RocketsimWrapper>) -> CompatWrapper{
// //         CompatWrapper { reward_fn, obs_builder, sim_wrapper }
// //     }
// // }

// #[pymethods]
// impl CompatObs {
//     #[new]
//     pub fn new(tick_skip: usize, spawn_opponents: bool, team_size: usize) -> CompatObs {
//         // replace your game config here as necessary
//         let game_config = GameConfig {
//             tick_skip,
//             spawn_opponents,
//             team_size,
//             gravity: 1.,
//             boost_consumption: 1.,
//         };
//         let obs_builder = Box::new(AdvancedObs::new());
//         let simulator = RocketsimWrapper::new(game_config);
//         let previous_actions: Vec<Vec<f32>> = vec![vec![]];
//         CompatObs{
//             obs_builder,
//             previous_actions,
//             simulator,
//             game_config,
//         }
//     }
//     pub fn reset(&mut self, py_state: CompatGameState){
//         let gamestate = make_sim_state(py_state);
//         let state = self.simulator.get_rlgym_gamestate(false).0;
//         self.obs_builder.reset(&state);
//     }

//     fn pre_step(&mut self, py_state: CompatGameState, previous_actions: PyReadonlyArray<f32, Ix2>){
//         let gamestate = make_sim_state(py_state);
//         let mut state = self.simulator.get_rlgym_gamestate(&gamestate).0;
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
//         let obs_array = PyArray::from_vec(py, obs).into();
//         Ok(obs_array)
//     }
// }
