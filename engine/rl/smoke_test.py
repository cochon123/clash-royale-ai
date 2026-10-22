"""Integration checks against the actual native engine, not a mock simulator."""
import json
import numpy as np
from gymnasium.utils.env_checker import check_env

from .native_clash import NativeClashEnv, sample_affordable_action


def main():
    with NativeClashEnv(opponent="wait", ticks_per_step=1, max_ticks=2) as env:
        obs, _ = env.reset(seed=123)
        assert obs["card_ids"][0, 0] == 27000002
        assert env.raw_observation["hands"][0][0]["name"] == "Mortar"
        assert env.observation_space.contains(obs)
        first, reward, done, truncated, _ = env.step(np.array([0, 0, 0]))
        assert first["tick"][0] == 3 and reward == 0 and not done and not truncated
        _, reward, done, truncated, _ = env.step(np.array([0, 0, 0]))
        assert reward == 0 and not done and truncated
        try:
            env.step(np.array([0, 0, 0]))
        except RuntimeError:
            pass
        else:
            raise AssertionError("completed episode accepted another step")
        replay, _ = env.reset(seed=123)
        for key in obs:
            np.testing.assert_array_equal(obs[key], replay[key])
    with NativeClashEnv() as env:
        check_env(env, skip_render_check=True)
        obs, _ = env.reset(seed=1)
        rng = np.random.default_rng(1000001)
        for step in range(360):
            action = np.array(sample_affordable_action(env.raw_observation, 0, rng))
            obs, reward, done, truncated, info = env.step(action)
            assert env.observation_space.contains(obs)
            if done or truncated:
                assert done and not truncated, "expected genuine native completion"
                winner = env.raw_observation["winner_side"]
                assert reward == (1 if winner == 0 else -1 if winner == 1 else 0)
                print(json.dumps({"gymnasium_check": "passed", "native_terminal_tick":env.raw_observation["tick"], "winner_side":winner, "reward":reward, "decision_steps":step+1}))
                break
        else:
            raise AssertionError("episode did not reach native completion")


if __name__ == "__main__":
    main()
