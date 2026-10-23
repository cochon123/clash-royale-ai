"""Forward and backward timing for BoardHogPolicy at batch 256.

Run from ``engine``:

    ../.venv/bin/python -m rl.bench_board_policy
"""
from __future__ import annotations

import time

import torch

from .banana_policy import BananaEpisode
from .board_policy import BoardHogPolicy
from .test_banana_tower_hp import state
from .test_board_policy import sample_board


def _one_cache(policy: BoardHogPolicy):
    deck = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")
    hands = [[dict(slot=i, name=card, cost=cost, available=True)
              for i, (card, cost) in enumerate(zip(deck[:4], [4, 4, 2, 1]))]] * 2
    episode = BananaEpisode((deck, deck), battle_id="bench")
    observation = dict(hands=hands, elixir=[10.0, 9.0], tower_state=state(), board=sample_board(0))
    action = policy.act_batch([(episode, observation, 0, 0.0)], deterministic=True, placement_top_k=576)[0]
    return action.input_cache


def _repeat_context(cache, length: int):
    repeated = dict(cache)
    inputs = list(cache["inputs"])
    continuous, cards = inputs[0], inputs[1]
    if continuous.shape[0] < length:
        inputs[0] = continuous[:1].expand(length, -1).clone()
        inputs[1] = cards[:1].expand(length).clone()
        repeated["inputs"] = tuple(inputs)
    return repeated


def _benchmark(policy: BoardHogPolicy, cache, label: str) -> None:
    batch = 256
    caches = [cache] * batch
    slots = torch.zeros(batch, dtype=torch.long, device=policy.device)
    torch.cuda.reset_peak_memory_stats()

    def timed_step():
        # The GRU backward has to stay inside the PPO backend: cuDNN rejects it
        # once the encoder returns to the eval mode used for rollout.
        policy.zero_grad(set_to_none=True)
        with policy._ppo_backend():
            torch.cuda.synchronize()
            start = time.perf_counter()
            first = policy._forward_batch(caches)
            second = policy._placement_from_context(first, caches, list(range(batch)), slots)
            values = policy.critic(first["fused"]).squeeze(-1)
            torch.cuda.synchronize()
            mid = time.perf_counter()
            loss = first["slot_logits"].sum() + second["tile_logits"].sum() + values.sum()
            loss.backward()
            torch.cuda.synchronize()
            end = time.perf_counter()
        return (mid - start) * 1000, (end - mid) * 1000

    for _ in range(2):
        timed_step()
    forward_ms, backward_ms = [], []
    for _ in range(5):
        forward, backward = timed_step()
        forward_ms.append(forward)
        backward_ms.append(backward)
    peak = torch.cuda.max_memory_allocated() / (1024 ** 2)

    def mean(values):
        return sum(values) / len(values)

    print(
        f"{label} batch={batch} context={tuple(cache['inputs'][0].shape)} "
        f"forward_ms={[round(value, 2) for value in forward_ms]} mean_forward_ms={mean(forward_ms):.2f} "
        f"backward_ms={[round(value, 2) for value in backward_ms]} mean_backward_ms={mean(backward_ms):.2f} "
        f"mean_total_ms={mean(forward_ms) + mean(backward_ms):.2f} peak_mib={peak:.1f}"
    )


def main() -> None:
    if not torch.cuda.is_available():
        raise SystemExit("CUDA is not available")
    torch.set_num_threads(2)
    policy = BoardHogPolicy(device="cuda")
    cache = _one_cache(policy)
    report = policy.parameter_report()
    print("parameters", report)
    print("opening_context", tuple(cache["inputs"][0].shape), "entities", int(cache["board"].mask.sum()))
    for label, length in (("context_1", None), ("context_64", 64)):
        row = cache if length is None else _repeat_context(cache, length)
        _benchmark(policy, row, label)


if __name__ == "__main__":
    main()
