"""Named feature layouts so callers do not index magic positions."""

from __future__ import annotations

CONTINUOUS_FEATURE_NAMES: tuple[str, ...] = (
    "seconds",
    "side_flag",
    "is_double",
    "is_triple",
    "x",
    "y",
    "team_elixir",
    "opp_elixir",
    "elixir_diff",
    "cost",
    "is_ability",
    "is_spell",
    "is_wincon",
    "team_spent",
    "opp_spent",
    "spent_diff",
    "plays_diff",
    "deep_diff",
    "is_deep",
    "is_bridge",
    "streak",
    "dt_same",
    "recent_team_30",
    "recent_opp_30",
    "recent_diff_30",
    "cycle_avg",
    "acting_elixir",
    "pad",
)

GLOBAL_FEATURE_NAMES: tuple[str, ...] = (
    "team_spent",
    "opp_spent",
    "spent_diff",
    "team_plays",
    "opp_plays",
    "plays_diff",
    "team_deep",
    "opp_deep",
    "deep_diff",
    "team_left",
    "opp_left",
    "team_bridge",
    "opp_bridge",
    "team_mean_y",
    "opp_mean_y",
    "mean_y_diff",
    "team_abilities",
    "opp_abilities",
    "team_elixir",
    "opp_elixir",
    "elixir_diff",
    "now",
    "spend_diff_15",
    "spend_diff_30",
    "spend_diff_60",
    "team_15",
    "opp_15",
    "team_30",
    "opp_30",
    "deep_diff_30",
    "deep_diff_60",
    "last_deep_side",
    "team_since_deep",
    "opp_since_deep",
    "double_spent_diff",
    "triple_spent_diff",
    "spell_diff",
    "wincon_diff",
    "team_cycle_avg",
    "opp_cycle_avg",
    "cycle_avg_diff",
    "team_unique",
    "opp_unique",
    "team_max_deep_streak",
    "opp_max_deep_streak",
    "avg_cost_diff",
    "spell_frac_diff",
    "shared",
)

THREAT_FEATURE_NAMES: tuple[str, ...] = (
    "hog_rider_active",
    "hog_rider_age",
    "balloon_active",
    "balloon_age",
    "graveyard_active",
    "graveyard_age",
    "golem_active",
    "golem_age",
    "any_focus",
    "focus_cost",
    "any_wincon",
    "wincon_age",
    "threat_x",
    "threat_y",
)

CONTINUOUS_INDEX = {name: index for index, name in enumerate(CONTINUOUS_FEATURE_NAMES)}
GLOBAL_INDEX = {name: index for index, name in enumerate(GLOBAL_FEATURE_NAMES)}
THREAT_INDEX = {name: index for index, name in enumerate(THREAT_FEATURE_NAMES)}
CONTINUOUS_DIM = len(CONTINUOUS_FEATURE_NAMES)
GLOBAL_DIM = len(GLOBAL_FEATURE_NAMES)
THREAT_DIM = len(THREAT_FEATURE_NAMES)

# Official elixir/leak columns appended by the full-game winner judge. Sequence-only
# judges omit these because generated battles have no RoyaleAPI elixir tables.
OFFICIAL_ELIXIR_FEATURE_NAMES: tuple[str, ...] = (
    "official_team_total",
    "official_opp_total",
    "official_total_diff",
    "official_team_leak",
    "official_opp_leak",
    "official_leak_diff",
    "official_troop_diff",
    "official_spell_diff",
    "official_building_diff",
    "official_count_diff",
)
