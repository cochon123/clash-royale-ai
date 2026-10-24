#!/usr/bin/env python3
"""Small real-engine integration check, not a strength evaluation.

Uses two selection jobs and two disjoint confirmation jobs per decode mode
to exercise checkpoint selection, regression guards, and final loading.
Pass ordinary crazy_hog_train arguments. Run under the same resource cgroup.
"""
from pathlib import Path

from rl.banana_hog_retrain import main
from rl.board_policy import BoardHogPolicy
from rl.crazy_hog_train import CrazyHogLeague, configure_parser


class SmokeLeague(CrazyHogLeague):
    def eval_jobs(self, bank):
        jobs = super().eval_jobs(bank)
        # Distinct archetypes, controllers and seats, preserving bank seeds.
        return [jobs[0], next(job for job in jobs if job["deck_id"] == "pekka-bridge"
                             and job["controller"] == "bc-sampled" and job["seat"] == 1)]

    def run(self):
        try:
            self.status = "running"
            rows, transitions = self.collect_training()
            assert any(row["result"] != "VOID" for row in rows)
            self.update(transitions)
            for bank in ("selection", "confirmation"):
                assert self.evaluate(bank, parent=True), bank
                assert self.evaluate(bank), bank
            self.save_selected_final()
            loaded = BoardHogPolicy.from_checkpoint(self.run_dir / "models/final.pt", device="cpu")
            assert loaded.row_vocab.names == self.learner.row_vocab.names
            self.status, self.phase = "completed", "small integration smoke passed; not a strength claim"
            self.save("latest")
            self.publish(force=True)
        except BaseException as exc:
            self.status, self.phase = "failed", repr(exc)
            self.publish(force=True)
            raise
        finally:
            self.close_engines()
            self.executor.shutdown(wait=True, cancel_futures=True)
            self._finish_notes()
            from rl.crazy_hog_report import build_report
            build_report(self.run_dir, self.run_dir / "report.html")


if __name__ == "__main__":
    main(SmokeLeague, configure_parser)
