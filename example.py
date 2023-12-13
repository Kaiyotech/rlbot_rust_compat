from typing import Any

import numpy as np
import rlbot_rust_compat
import rlgym_sim
from gym import Space
from rlgym_sim.utils.gamestates import PlayerData, GameState, PhysicsObject

from rlgym_sim.utils.obs_builders import ObsBuilder
from rlgym_sim.utils.terminal_conditions.common_conditions import GoalScoredCondition, TimeoutCondition
from rlgym_sim.utils.state_setters.default_state import DefaultState
from example_parser import LookupAction
from datetime import datetime


class TestObsBuilder(ObsBuilder):
    def __init__(self,
                 tick_skip=8,
                 team_size=3,
                 spawn_opponents=True,
                 ):
        super().__init__()
        self.index = 0
        self.rust_obs_builder = rlbot_rust_compat.CompatObs(tick_skip=tick_skip,
                                                            team_size=team_size,
                                                            spawn_opponents=spawn_opponents)

    def reset(self, initial_state: GameState):
        self.rust_obs_builder.reset(initial_state)

    def pre_step(self, state: GameState, previous_actions: np.ndarray):
        self.rust_obs_builder.pre_step(state, previous_actions)

    def build_obs(self, player: PlayerData, state: GameState, _previous_action: np.ndarray) -> Any:
        obs = self.rust_obs_builder.build_obs(player, state, _previous_action)
        return obs

    # def get_obs_space(self) -> Space:
    #


if __name__ == "__main__":
    startTime = datetime.now()
    terminals = [GoalScoredCondition(), TimeoutCondition(200)]
    parser = LookupAction()
    setter = DefaultState()
    obs_builder = TestObsBuilder()
    env = rlgym_sim.make(tick_skip=4, spawn_opponents=True, copy_gamestate_every_step=True,
                         terminal_conditions=terminals, action_parser=parser, team_size=2,
                         state_setter=setter, obs_builder=obs_builder)
    total_steps = 0
    num_episodes_to_do = 100
    episodes_done = 0
    while episodes_done <= num_episodes_to_do:
        done = False
        steps = 0
        env.reset()
        while not done:
            # actions = np.asarray((np.asarray([0]), np.asarray([np.random.randint(0, 373)])))
            # actions = np.asarray(np.asarray([0],))
            # actions = np.asarray([0] * 8), np.asarray([0] * 8)
            # actions = np.asarray(
            #     [np.asarray([1, 0.5, 0.5, 0.5, 0, 0, 1, 0]), np.asarray([1, 0.5, 0.5, 0.5, 0, 0, 1, 0])])
            actions = np.asarray(([0], [0], [0], [0]))
            new_obs, reward, done, game_state = env.step(actions)
            obs = new_obs
            steps += 1
        total_steps += steps
        episodes_done += 1
        # print(f"completed {steps} steps. Starting new episode. Done {total_steps} total steps")

    print(f"executed {total_steps} steps in {datetime.now() - startTime}")