from typing import Any

import rlbot_rust_compat
from rlgym_sim.utils.gamestates import PlayerData, GameState, PhysicsObject

from rlgym_sim.utils.obs_builders import ObsBuilder

import numpy as np

from gym import Space
from gym.spaces import Tuple, Box


class TestObsBuilder(ObsBuilder):
    def __init__(self,
                 tick_skip=8,
                 ):
        super().__init__()
        self.tick_skip = tick_skip
        self.index = 0
        self.rust_obs_builder = my_rust.ObsBuilder(tick_skip = 4, infinite_boost_odds = 0)

    def reset(self, initial_state: GameState):
        self.rust_obs_builder.reset(initial_state)

    def pre_step(self, state: GameState):
        self.rust_obs_builder.pre_step(state)

    def build_obs(self, player: PlayerData, state: GameState, previous_action: np.ndarray, previous_model_actions: np.ndarray) -> Any:
        obs = self.rust_obs_builder.build_obs(state, previous_action)
        return obs

    def get_obs_space(self) -> Space:
        players = 5
        car_size = 35
        player_size = 10
        return Box(-np.inf, np.inf, (1, 10))