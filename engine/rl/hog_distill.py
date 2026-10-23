"""Distill a board-aware teacher into the board-free Hog policy.

The teacher is ``BoardHogPolicy``. The student is ``AdaptiveHogPolicy``, which
is what a phone deployment can condition on today: hands, elixir, play history
and tower HP, with no entity board. The loss is KL(teacher || student) on the
wait Bernoulli, the masked card-slot categorical, and the full 576-tile
placement categorical. Temperatures are the ones stored on each rollout cache.

Rollout files are produced with ``torch.save``::

    {"format": "hog-distill-rollout-v1",
     "caches": [action.input_cache, ...],
     "placement_slots": [action.deck_slot, ...]}

Caches already contain the board tensors captured at decision time. The
student ignores that field. Placement slots are the deck slots whose tile
distributions are matched; use the action that was actually played, or the
teacher's chosen slot for waits.
"""
from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any, Mapping, Sequence

import torch
from torch import Tensor
from torch.distributions import Bernoulli, Categorical, kl_divergence

from .banana_hog_adaptive import AdaptiveHogPolicy
from .board_policy import DEFAULT_HOG2_WEIGHTS, BoardHogPolicy

ROLLOUT_FORMAT = "hog-distill-rollout-v1"


def _as_slots(placement_slots: Tensor | Sequence[int], device: torch.device) -> Tensor:
    if isinstance(placement_slots, Tensor):
        return placement_slots.to(device=device, dtype=torch.long)
    return torch.tensor(list(placement_slots), dtype=torch.long, device=device)


def policy_action_logits(
    policy,
    caches: Sequence[Mapping[str, Any]],
    placement_slots: Tensor | Sequence[int],
) -> dict[str, Tensor]:
    """Sampling distributions' logits for wait, the eight card slots, and 576 tiles."""
    first = policy._forward_batch(caches)
    wait_logits = policy.wait_head(first["fused"]).squeeze(-1)
    slot_temperature = wait_logits.new_tensor([
        max(float(cache.get("temperature", 1.0)), 1e-4) for cache in caches
    ])
    tile_temperature = wait_logits.new_tensor([
        max(float(cache.get("placement_temperature", 0.8)), 1e-4) for cache in caches
    ])
    mask = first["_hand_mask"]
    slot_logits = first["slot_logits"] / slot_temperature[:, None]
    slot_logits = slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min)
    slots = _as_slots(placement_slots, wait_logits.device)
    if slots.shape[0] != wait_logits.shape[0]:
        raise ValueError("placement_slots must match the cache batch")
    second = policy._placement_from_context(first, caches, list(range(len(caches))), slots)
    tile_logits = second["tile_logits"] / tile_temperature[:, None]
    return {"wait": wait_logits, "slots": slot_logits, "tiles": tile_logits}


def distribution_kl(
    teacher_wait: Tensor,
    student_wait: Tensor,
    teacher_slots: Tensor,
    student_slots: Tensor,
    teacher_tiles: Tensor,
    student_tiles: Tensor,
) -> dict[str, Tensor]:
    """KL(teacher || student), averaged over the batch, for the three heads."""
    wait = kl_divergence(Bernoulli(logits=teacher_wait), Bernoulli(logits=student_wait))
    slots = kl_divergence(Categorical(logits=teacher_slots), Categorical(logits=student_slots))
    tiles = kl_divergence(Categorical(logits=teacher_tiles), Categorical(logits=student_tiles))
    return {
        "wait": wait.mean(),
        "slots": slots.mean(),
        "tiles": tiles.mean(),
        "total": (wait + slots + tiles).mean(),
    }


def distill_batch(
    teacher,
    student,
    caches: Sequence[Mapping[str, Any]],
    placement_slots: Tensor | Sequence[int],
) -> tuple[Tensor, dict[str, Tensor]]:
    """One KL step. Teacher logits are detached. Student logits keep their graph."""
    with torch.no_grad():
        target = policy_action_logits(teacher, caches, placement_slots)
    current = policy_action_logits(student, caches, placement_slots)
    stats = distribution_kl(
        target["wait"], current["wait"], target["slots"], current["slots"], target["tiles"], current["tiles"],
    )
    return stats["total"], stats


def load_rollout(path: str | Path) -> tuple[list, list]:
    payload = torch.load(Path(path), map_location="cpu", weights_only=False)
    if payload.get("format") != ROLLOUT_FORMAT:
        raise ValueError(f"expected {ROLLOUT_FORMAT}")
    caches = list(payload["caches"])
    slots = list(payload["placement_slots"])
    if not caches or len(caches) != len(slots):
        raise ValueError("rollout caches and placement_slots must be non-empty and aligned")
    return caches, slots


def train_student(
    teacher: BoardHogPolicy,
    student: AdaptiveHogPolicy,
    caches: Sequence[Mapping[str, Any]],
    placement_slots: Sequence[int],
    *,
    steps: int,
    lr: float,
    batch_size: int,
) -> list[float]:
    if steps < 1:
        raise ValueError("steps must be positive")
    teacher.eval()
    # Stay in eval so distillation matches the dropout-free PPO logits.
    # Gradients still flow into the student.
    student.eval()
    optimizer = torch.optim.Adam([parameter for parameter in student.parameters() if parameter.requires_grad], lr=lr)
    losses: list[float] = []
    for step in range(steps):
        start = (step * batch_size) % len(caches)
        index = [(start + offset) % len(caches) for offset in range(min(batch_size, len(caches)))]
        batch = [caches[i] for i in index]
        slots = [placement_slots[i] for i in index]
        optimizer.zero_grad(set_to_none=True)
        loss, _stats = distill_batch(teacher, student, batch, slots)
        loss.backward()
        optimizer.step()
        losses.append(float(loss.detach()))
    return losses


def save_student(student: AdaptiveHogPolicy, path: str | Path) -> None:
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    torch.save(
        {
            "format": "banana-native-ppo-v1",
            "model_state": student.model.state_dict(),
            "wait_state": student.wait_head.state_dict(),
            "critic_state": student.critic.state_dict(),
            "name": "hog-distill-student",
            "updates": int(getattr(student, "checkpoint_updates", 0)),
        },
        path,
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Distill BoardHogPolicy into AdaptiveHogPolicy")
    parser.add_argument("--teacher", type=Path, default=None, help="board-hog-policy-v1 checkpoint")
    parser.add_argument("--teacher-weights", type=Path, default=DEFAULT_HOG2_WEIGHTS, help="used when --teacher is omitted")
    parser.add_argument("--student-weights", type=Path, default=DEFAULT_HOG2_WEIGHTS)
    parser.add_argument("--rollout", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--steps", type=int, default=1)
    parser.add_argument("--batch-size", type=int, default=32)
    parser.add_argument("--lr", type=float, default=1e-4)
    parser.add_argument("--trainable", choices=("all", "placement", "heads"), default="all")
    parser.add_argument("--device", default="cpu")
    return parser


def main(argv: Sequence[str] | None = None) -> list[float]:
    args = build_parser().parse_args(argv)
    if args.teacher is not None:
        teacher = BoardHogPolicy.from_checkpoint(args.teacher, device=args.device)
    else:
        teacher = BoardHogPolicy(weights=args.teacher_weights, device=args.device)
    student = AdaptiveHogPolicy(weights=args.student_weights, device=args.device, trainable=args.trainable)
    caches, slots = load_rollout(args.rollout)
    losses = train_student(
        teacher, student, caches, slots, steps=args.steps, lr=args.lr, batch_size=args.batch_size,
    )
    save_student(student, args.out)
    print(f"steps={len(losses)} final_kl={losses[-1]:.6f} out={args.out}")
    return losses


if __name__ == "__main__":
    main()
