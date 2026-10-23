"""Elixir-trade potential (C6).

Phi is unscaled:

    Phi = (own_elixir + own_board_value) - (enemy_elixir + enemy_board_value)

``own_*`` is the learner, the same side passed as ``perspective``. Board value
sums ``card_cost / units_per_card * hp / max_hp`` over non-tower entities.
Towers stay in the separate tower-HP potential. Unknown rows contribute 0.

Shaping follows ``banana_hog_retrain.shaped_rewards``. For a trajectory
``potentials`` of length ``decisions + 1``, the last stored potential is the
measured end state and is replaced by terminal Phi = 0:

    states = potentials[:-1] + [0]
    shaped[i] = coefficient * (states[i + 1] - states[i])

Those terms telescope. With terminal Phi fixed at 0:

    sum(shaped) = -coefficient * potentials[0]

The win/loss outcome is not added here. The trainer adds it once, on the last
reward, exactly as it already does for tower HP.
"""
from __future__ import annotations

import math
from dataclasses import dataclass
from typing import Any, Mapping, Sequence

from .board_features import RowTable, board_values, load_row_table


@dataclass(frozen=True)
class ElixirTrade:
    potential: float
    own_elixir: float
    enemy_elixir: float
    own_board_value: float
    enemy_board_value: float
    unknown_entities: int
    perspective: int


def _elixir(value: Any, label: str) -> float:
    try:
        number = float(value)
    except (TypeError, ValueError) as error:
        raise ValueError(f"{label} elixir is not a number") from error
    if not math.isfinite(number):
        raise ValueError(f"{label} elixir is not finite")
    return number


def elixir_trade_potential(
    board: Mapping[str, Any],
    perspective: int,
    own_elixir: float,
    enemy_elixir: float,
    table: RowTable | None = None,
) -> ElixirTrade:
    """Learner-perspective elixir trade. ``own_elixir`` is ``elixir[perspective]``."""
    table = load_row_table() if table is None else table
    own_board, enemy_board, unknown = board_values(board, perspective, table)
    own = _elixir(own_elixir, "own")
    enemy = _elixir(enemy_elixir, "enemy")
    return ElixirTrade(
        potential=(own + own_board) - (enemy + enemy_board),
        own_elixir=own,
        enemy_elixir=enemy,
        own_board_value=own_board,
        enemy_board_value=enemy_board,
        unknown_entities=unknown,
        perspective=perspective,
    )


def shaped_elixir_terms(potentials: Sequence[float], coefficient: float = 1.0) -> list[float]:
    """Potential differences with the terminal state forced to Phi = 0."""
    if len(potentials) < 2:
        raise ValueError("elixir shaping needs the initial potential and one successor")
    coefficient = _elixir(coefficient, "coefficient")
    states = [float(value) for value in potentials[:-1]] + [0.0]
    return [coefficient * (after - before) for before, after in zip(states, states[1:])]


def audit_elixir_trade(
    potentials: Sequence[float],
    coefficient: float = 1.0,
    terms: Sequence[float] | None = None,
    tol: float = 1e-6,
) -> dict[str, Any]:
    """Verify ``sum(shaped) == -coefficient * Phi_initial`` under terminal Phi = 0.

    Pass ``terms`` to also check a trainer's shaped series against the series
    derived from ``potentials``. A mismatch raises.
    """
    expected_terms = shaped_elixir_terms(potentials, coefficient)
    if terms is not None:
        supplied = [float(value) for value in terms]
        if len(supplied) != len(expected_terms) or any(abs(a - b) > tol for a, b in zip(supplied, expected_terms)):
            raise RuntimeError("shaped elixir terms do not match the potential trajectory")
    total = float(sum(expected_terms))
    initial = float(potentials[0])
    expected = -float(coefficient) * initial
    if abs(total - expected) > tol:
        raise RuntimeError(
            f"elixir-trade potential failed telescoping: sum={total} expected={expected}"
        )
    return {
        "ok": True,
        "sum_shaped": total,
        "expected": expected,
        "initial_potential": initial,
        "coefficient": float(coefficient),
        "steps": len(expected_terms),
        "terminal_potential": 0.0,
    }
