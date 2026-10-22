"""All-archetype Hog PPO, continuing a saved tower-aware specialist.

Uses the original ARM engine for the entire pool. The frozen Hog parent and
original BC actor provide three opponent styles. Unseen native seeds, rather
than omitted archetypes, provide the final holdout.
"""
from __future__ import annotations

import json
from pathlib import Path

from .banana_hog_adaptive import AdaptiveHogPolicy, AdaptiveLeague, configure_parser
from .banana_hog_retrain import main as stable_main
from .banana_train import ROOT, sha256


class OvernightLeague(AdaptiveLeague):
    @property
    def evaluation_controllers(self):
        return ("bc-greedy", "bc-sampled", "hog1-greedy")

    def make_parent_policy(self, **kwargs):
        # The parent already has HP residual weights. The original SamplingPolicy
        # loader cannot load them; use exactly the learner's architecture.
        return AdaptiveHogPolicy(trainable="heads", **kwargs)

    def make_sampler(self, decks, seed):
        for deck in decks:
            deck["holdout"] = False
        return super().make_sampler(decks, seed)

    def manifest_extra(self):
        result = super().manifest_extra()
        result.update(
            format="banana-hog-overnight-v1",
            overnight_source_sha256=sha256(Path(__file__)),
            engine_contract="Original recovered ARM engine through Unicorn for all eleven archetypes; no approximate simulator or uncovered compiled paths.",
            inference_contract="PPO Bernoulli wait controller with full-grid native legal masks. Use --legacy-wait-head when watching; learned phone-delay scheduling is a different policy.",
            holdout_contract="All eleven archetypes train; final confirmation uses disjoint native seeds and never feeds back into learning.",
        )
        result["experiment"]["description"] = "Overnight Hog 2.6 from the saved Hog specialist against eleven classic archetypes and three frozen bot controllers"
        result["experiment"]["notes"] = [
            "No live game, phone, account, external AI API, or paid compute is used.",
            "Recovered base cards exclude modern evolutions, heroes and balance changes.",
            "The actor sees deck lists, accepted play history, hands, elixir and tower HP, but no troop positions or unit HP.",
            "Frozen bots are not expert deck pilots; strength must be measured.",
            "The exact saved parent remains the fallback if no candidate passes selection.",
        ]
        return result

    def extra_state(self):
        result = super().extra_state()
        result["notes"] = self.manifest_extra()["experiment"]["notes"] + [
            "All eleven archetypes participate in training. Final holdout is by native seed.",
            "Sampled PPO performance selects the model; greedy play is also checked.",
        ]
        return result

    def restore(self, path):
        manifest = self.run_dir / "manifest.json"
        if manifest.exists():
            saved = json.loads(manifest.read_text())
            if saved.get("overnight_source_sha256") != sha256(Path(__file__)):
                raise ValueError("overnight trainer changed; refusing a silent resume")
        super().restore(path)


def parser_config(parser):
    configure_parser(parser)
    parser.set_defaults(
        name="banana-hog-overnight-20261022",
        parent=ROOT / "runs/banana-hog-1/models/final.pt",
        parent_name="banana-hog-1-selected-v65",
        opponents="original",
        eval_every=3600,
        eval_seeds=2,
        confirmation_seeds=4,
        regression_threshold=.08,
    )


if __name__ == "__main__":
    stable_main(OvernightLeague, parser_config)
