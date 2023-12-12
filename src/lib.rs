pub mod bytes; // replace this with rocketsim_rs with bin feature somehoow? Stuff is private though
pub mod gym;
pub mod sim_state;
pub mod gym_state;

pub use gym::{
    CompatGameState,
    CompatPlayerData,
    BOOST_PADS_LENGTH
};

pub use sim_state::{make_sim_state};

pub use gym_state::{RocketsimWrapper,
    get_car_controls_from_vec
};

pub use bytes::Vec3;