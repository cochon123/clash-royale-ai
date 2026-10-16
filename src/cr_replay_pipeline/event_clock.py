"""Joint event clock: competing-risk rates and quiet-interval likelihood.

Piecewise-constant own/opponent hazards on a 250 ms grid collapse to an
exponential waiting-time NLL when the rates are held fixed between events.
New training must provide both actor labels under a fixed perspective.
The optional own default exists only to evaluate archived v8 artifacts.
"""

from __future__ import annotations

import torch
import torch.nn.functional as F

CLOCK_HORIZONS = (2.0, 5.0, 10.0)
GRID_SECONDS = 0.25


def positive_rates(raw: torch.Tensor) -> torch.Tensor:
    """Map unconstrained logits to strictly positive event rates."""
    return F.softplus(raw) + 1e-4


def competing_risk_nll(
    rates: torch.Tensor,
    delay_seconds: torch.Tensor,
    event_side: torch.Tensor | None = None,
    observed: torch.Tensor | None = None,
) -> torch.Tensor:
    """Negative log p(next event at t, side s) under constant competing rates.

    ``rates`` is ``(B, 2)`` as ``(λ_own, λ_opp)``. ``delay_seconds`` is the
    observed wait after the last prefix event. Survival is
    ``exp(-(λ_own+λ_opp) t)``; the event density contributes ``λ_s``.
    """
    delay = delay_seconds.clamp_min(0.0)
    total = rates.sum(dim=-1)
    if event_side is None:
        event_rate = rates[:, 0]
    else:
        side = event_side.long().clamp(0, 1)
        event_rate = rates.gather(1, side.unsqueeze(1)).squeeze(1)
        # A timestamp group with both actors has a known event time but an
        # unobserved first actor. Marginalize its cause, do not censor the event.
        event_rate = torch.where(event_side < 0, total, event_rate)
    event_term = event_rate.log()
    if observed is not None:
        event_term = event_term * observed.to(event_term.dtype)
    return total * delay - event_term


def activity_targets(
    delay_seconds: torch.Tensor,
    horizons: tuple[float, ...] = CLOCK_HORIZONS,
) -> torch.Tensor:
    """Binary labels: whether the next event occurs within each horizon."""
    delay = delay_seconds.clamp_min(0.0).unsqueeze(-1)
    cuts = delay.new_tensor(list(horizons)).view(1, -1)
    return (delay <= cuts).float()
