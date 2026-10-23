"""Continuous collection: a worker starts the next game when its game ends.

PPO still consumes stored behavior log-probs. Decisions are tagged with the
policy version that sampled them. A finished game whose oldest decision is
more than ``max_policy_lag`` versions behind the learner is kept in the match
log and reward audit, and left out of the update.
"""
from __future__ import annotations

from concurrent import futures


def add_async_args(parser):
    parser.add_argument("--async-collection", action="store_true",
                        help="Start a new game as soon as a worker finishes. PPO runs when enough games are ready. Default keeps wave barriers.")
    parser.add_argument("--max-policy-lag", type=int, default=2,
                        help="Drop completed games whose oldest decision is more than this many policy versions behind.")
    parser.add_argument("--async-ready-games", type=int, default=0,
                        help="Completed non-stale games to gather before PPO. 0 uses the worker count.")


def policy_lag(actions, version):
    """How many learner versions behind the oldest tagged decision is."""
    known = [int(getattr(action, "behavior_version")) for action in actions
             if getattr(action, "behavior_version", None) is not None]
    if not known:
        return 0
    return int(version) - min(known)


def within_policy_lag(actions, version, max_lag):
    return policy_lag(actions, version) <= int(max_lag)


class ContinuousCollector:
    """Keep ``workers`` games in flight. ``collect`` returns one PPO batch.

    Games that are still stepping stay in this object across PPO updates and
    keep the behavior version stored on each decision.
    """

    def __init__(self, league):
        self.league = league
        self.active = {}
        self.step_jobs = {}
        self.reset_jobs = {}
        self.ready = []

    def close(self):
        pending = [*self.step_jobs, *self.reset_jobs]
        if pending:
            futures.wait(pending, timeout=45)

    def collect(self, target_games):
        league = self.league
        rows, transitions = [], []
        usable = 0
        while usable < target_games and not league.stopping():
            self._start_idle()
            self._drain_resets(rows)
            # Launch one step-group, then another, before blocking. The second
            # group's inference overlaps the first group's engine step. A game
            # that finishes is reset on the next pass without waiting for its sibling.
            if self.ready:
                self._launch_one_group()
                if self.ready:
                    continue
            if not self.step_jobs and not self.reset_jobs:
                break
            finished = self._wait_some()
            for kind, future in finished:
                if kind == "reset":
                    self._finish_reset(future, rows)
                elif self._finish_step(future, rows, transitions):
                    usable += 1
            league.publish()
        return rows, transitions

    def _launch_one_group(self):
        from .banana_hog_league import League
        count = max(1, int(getattr(self.league.args, "step_groups", 1)))
        if count == 1 or len(self.ready) == 1:
            batch = list(self.ready)
            self.ready = []
        else:
            groups = League._split_step_groups(self.ready, count, min(2, max(1, len(self.ready) // count)))
            batch = list(groups[0])
            launched = {game.worker for game in batch}
            self.ready = [game for game in self.ready if game.worker not in launched]
        self.league._launch_steps(batch, self.league.learner, True, self.step_jobs)

    def _drain_resets(self, rows):
        if not self.reset_jobs:
            return
        done, _ = futures.wait(self.reset_jobs, timeout=0, return_when=futures.FIRST_COMPLETED)
        for future in done:
            self._finish_reset(future, rows)

    def _idle_workers(self):
        busy = set(self.active)
        busy.update(game.worker for game in self.ready)
        busy.update(worker for worker, _job in self.reset_jobs.values())
        return [worker for worker in range(self.league.args.workers) if worker not in busy]

    def _start_idle(self):
        league = self.league
        for worker in self._idle_workers():
            if league.stopping():
                return
            job = league.sampler.draw()
            name = league.decks[job["deck_id"]]["name"]
            league.worker_states[worker] = {"id": worker, "status": "resetting", "deck": name,
                                            "controller": job["controller"], "seat": job["seat"]}
            future = league.executor.submit(league.reset, worker, job)
            self.reset_jobs[future] = (worker, job)

    def _wait_some(self):
        pending = set(self.step_jobs) | set(self.reset_jobs)
        done, _ = futures.wait(pending, timeout=0.5, return_when=futures.FIRST_COMPLETED)
        if not done:
            return []
        # Pick up siblings that already returned, and do not wait for a slower
        # game. A positive gather window pulled staggered workers back together
        # and put that wait on the critical path.
        pending = [future for future in self.step_jobs if future not in done]
        if pending:
            extra, _ = futures.wait(pending, timeout=0, return_when=futures.ALL_COMPLETED)
            done = set(done) | set(extra)
        step_done = [future for future in done if future in self.step_jobs]
        reset_done = [future for future in done if future in self.reset_jobs]
        return [("reset", future) for future in reset_done] + [("step", future) for future in step_done]

    def _finish_reset(self, future, rows):
        worker, job = self.reset_jobs.pop(future)
        league = self.league
        try:
            game = future.result()
        except Exception as exc:
            from .banana_hog_league import Game
            from .banana_policy import BananaEpisode
            decks = (tuple(league.hog["cards"]), tuple(league.decks[job["deck_id"]]["cards"]))
            dummy = Game(worker, job, BananaEpisode(decks), {"tick": 0}, 0)
            rows.append(league.record(dummy, "train", "VOID", str(exc)))
            league.restart_worker(worker)
            return
        self.active[worker] = game
        self.ready.append(game)

    def _finish_step(self, future, rows, transitions):
        """Return True when a non-stale terminal game was added to the batch."""
        game = self.step_jobs.pop(future)
        league = self.league
        before_rows, before_transitions = len(rows), len(transitions)
        alive = league._consume_step(game, future, "train", True, self.active, rows, transitions)
        if alive:
            self.ready.append(game)
            return False
        if len(rows) == before_rows:
            return False
        row = rows[-1]
        if row.get("result") == "VOID":
            return False
        if not within_policy_lag(getattr(game, "decisions", ()), league.version, league.args.max_policy_lag):
            del transitions[before_transitions:]
            league.counters["stale_games"] += 1
            league.log(f"dropped stale game {row.get('id')} lag {policy_lag(game.decisions, league.version)} > {league.args.max_policy_lag}")
            return False
        return True
