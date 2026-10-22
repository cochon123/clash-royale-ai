"""Scheduler tests exercise real launch/consume code with exclusive fake pipes."""
import concurrent.futures as futures
from collections import Counter
from contextlib import contextmanager
from dataclasses import dataclass
from threading import Event, Lock
from types import SimpleNamespace
from unittest import TestCase, main
from unittest.mock import patch

from .banana_hog_league import BananaEpisode, Game, League


@dataclass
class WaitAction:
    side: int
    tick: int
    wait: bool = True


class Client:
    def __init__(self, worker, trace):
        self.worker, self.trace = worker, trace
        self.lock = Lock()
        self.steps = 0
        self.limit = 3
        self.fail = False
        self.closed = False
        self.step_entered = Event()
        self.release_step = None

    def request(self, payload, timeout=45):
        assert self.lock.acquire(blocking=False), 'two outstanding requests on one NativeClient'
        try:
            assert not self.closed
            op = payload['op']
            self.trace.append((op, self.worker, self.steps))
            if op == 'reset':
                self.steps = 0
                self.limit = payload['limit']
                return {'tick': 100}
            if op == 'placement_mask':
                return []
            self.step_entered.set()
            if self.release_step is not None:
                assert self.release_step.wait(5), 'test engine step was never released'
            if self.fail:
                raise RuntimeError('fake_engine_error')
            self.steps += 1
            return {'observation': {'tick': 100 + 20 * self.steps, 'winner_side': 0},
                    'actions': [], 'terminated': self.steps >= self.limit, 'truncated': False}
        finally:
            self.lock.release()

    def close(self):
        assert self.lock.acquire(blocking=False), 'restart before request drained'
        self.closed = True
        self.lock.release()


class InlineExecutor:
    """Completed futures make the historical call-order comparison deterministic."""
    def submit(self, fn, *args, **kwargs):
        future = futures.Future()
        try:
            future.set_result(fn(*args, **kwargs))
        except Exception as exc:
            future.set_exception(exc)
        return future

    def shutdown(self, **kwargs):
        pass


@contextmanager
def scheduler(count=4, groups=2, inline=False):
    league = object.__new__(League)
    league.args = SimpleNamespace(workers=count, step_groups=groups)
    league.trace = []
    league.clients = [Client(worker, league.trace) for worker in range(count)]
    league.executor = InlineExecutor() if inline else futures.ThreadPoolExecutor(max_workers=count)
    cards = ['hog-rider', 'ice-golem', 'musketeer', 'cannon', 'skeletons', 'ice-spirit', 'fireball', 'the-log']
    league.decks = {'hog-mirror': {'name': 'Hog', 'cards': cards}}
    league.hog = {'cards': cards}
    league.worker_states = [{} for _ in range(count)]
    league.counters, league.fun = Counter(), {'card_plays': {}}
    league.parent_policy, league.base = object(), object()
    league.version = 7
    league.stop = False
    league.stopping = lambda: league.stop
    league.batches, league.restarts = [], []
    league.publish = lambda force=False: league.trace.append(('publish', force))

    def reset(worker, job):
        obs = league.clients[worker].request({'op': 'reset', 'limit': job['limit']})
        return Game(worker, job, object(), obs, 100)

    def record(game, kind, result, error=None):
        assert not league.clients[game.worker].lock.locked()
        league.trace.append(('record', game.worker, result, error))
        return {'worker': game.worker, 'result': result, 'error': error, 'version': league.version}

    def restart(worker):
        league.clients[worker].close()
        league.clients[worker] = Client(worker, league.trace)
        league.restarts.append(worker)

    def learner(policy, requests, games, collecting):
        league.batches.append(tuple(game.worker for game in games))
        league.trace.append(('infer', tuple(game.worker for game in games)))
        # Emulate concurrent masks on the SAME bounded executor as engine steps.
        masks = [league.executor.submit(league.clients[g.worker].request, {'op': 'placement_mask'}) for g in games]
        for future in masks:
            future.result(timeout=5)
        return [WaitAction(game.job['seat'], game.obs['tick']) for game in games]

    def after_step(game, reply):
        assert all(action.tick == game.obs['tick'] for _, action in game.pending_rows)
        assert reply['observation']['tick'] == game.obs['tick'] + 20
        league.trace.append(('consume', game.worker, game.obs['tick']))

    league.reset, league.record, league.restart_worker = reset, record, restart
    league.learner_actions = learner
    league.opponent_actions = lambda policy, requests, games, controller: [
        WaitAction(1 - game.job['seat'], game.obs['tick']) for game in games]
    league.after_native_step = after_step
    league.update = lambda *_: (_ for _ in ()).throw(AssertionError('optimizer ran inside wave'))
    try:
        yield league
    finally:
        for client in league.clients:
            if client.release_step:
                client.release_step.set()
        league.executor.shutdown(wait=True)


def jobs(*limits):
    return [dict(deck_id='hog-mirror', controller='bc-greedy', seat=0, seed=index, limit=limit)
            for index, limit in enumerate(limits)]


# Frozen historical scheduling loop, deliberately independent of grouped code.
def legacy_play(self, jobs, policy, kind, collecting=False):
    """Fixed-weight waves; no optimizer mutation while games are active."""
    completed, transitions = [], []
    for offset in range(0, len(jobs), self.args.workers):
        if self.stopping():
            break
        active = {}
        pending = {}
        for worker, job in enumerate(jobs[offset:offset + self.args.workers]):
            self.worker_states[worker] = {"id": worker, "status": "resetting", "deck": self.decks[job["deck_id"]]["name"],
                                          "controller": job["controller"], "seat": job["seat"]}
            pending[self.executor.submit(self.reset, worker, job)] = (worker, job)
        self.publish(force=True)
        while pending:
            done, _ = futures.wait(pending, timeout=.5, return_when=futures.FIRST_COMPLETED)
            self.publish()
            for future in done:
                worker, job = pending.pop(future)
                try:
                    active[worker] = future.result()
                except Exception as exc:
                    dummy = Game(worker, job, BananaEpisode((tuple(self.hog["cards"]), tuple(self.decks[job["deck_id"]]["cards"]))), {"tick": 0}, 0)
                    completed.append(self.record(dummy, kind, "VOID", str(exc)))
                    self.restart_worker(worker)
        step_jobs = {}
        ready = list(active.values())
        held = []  # barrier mode keeps early finishers until the whole wave returns
        while active:
            if self.stopping():
                if step_jobs:
                    futures.wait(step_jobs, timeout=45)
                    for future in list(step_jobs):
                        game = step_jobs.pop(future)
                        if game.worker in active:
                            self._consume_step(game, future, kind, collecting, active, completed, transitions)
                for game in list(active.values()):
                    completed.append(self.record(game, kind, "VOID", "deadline_or_stop"))
                break
            if ready:
                self._launch_steps(ready, policy, collecting, step_jobs)
                ready = []
            if not step_jobs:
                break
            done = self._completed_steps(step_jobs, pipeline=getattr(self.args, "pipeline", False))
            self.publish()
            finished_early = []
            for future in done:
                game = step_jobs.pop(future)
                if self._consume_step(game, future, kind, collecting, active, completed, transitions):
                    finished_early.append(game)
            if getattr(self.args, "pipeline", False):
                ready = finished_early
            else:
                held.extend(finished_early)
                if not step_jobs:
                    ready = held
                    held = []
            self.publish()
    return completed, transitions


class StepGroups(TestCase):
    def test_one_group_matches_historical_calls_and_order(self):
        # Stable Future iteration removes only the historical set-order noise.
        def done_in_order(pending, **kwargs):
            return list(pending), set()
        traces = []
        for play in (legacy_play, League.play):
            with scheduler(groups=1, inline=True) as league, patch(
                    'rl.banana_hog_league.futures.wait', side_effect=done_in_order):
                rows, transitions = play(league, jobs(2, 3, 1, 2), object(), 'train', True)
                traces.append((league.trace, rows, [(a.tick, outcome) for a, outcome in transitions]))
        self.assertEqual(traces[0], traces[1])

    def test_two_groups_complete_and_preserve_pending_rows(self):
        with scheduler() as league:
            rows, transitions = league.play(jobs(3, 3, 3, 3), object(), 'train', True)
            self.assertEqual([row['result'] for row in rows], ['W'] * 4)
            self.assertEqual(len(transitions), 12)
            self.assertEqual(league.batches, [(0, 1), (2, 3)] * 3)
            self.assertEqual(league.counters['decisions'], 24)
            self.assertEqual(league.pace['engine_steps'], 12)
            self.assertEqual(league.pace['infer_games'], 12)
            self.assertEqual({row['version'] for row in rows}, {7})

    def test_inference_overlaps_other_group_steps_with_bounded_executor(self):
        with scheduler() as league:
            release = Event()
            league.clients[0].release_step = release
            original = league.learner_actions

            def learner(policy, requests, games, collecting):
                if tuple(game.worker for game in games) == (2, 3) and not release.is_set():
                    self.assertTrue(league.clients[0].step_entered.wait(5))
                    self.assertTrue(league.clients[0].lock.locked())
                    actions = original(policy, requests, games, collecting)
                    release.set()
                    return actions
                return original(policy, requests, games, collecting)

            league.learner_actions = learner
            rows, _ = league.play(jobs(2, 2, 2, 2), object(), 'train', True)
            self.assertEqual(len(rows), 4)

    def test_stop_drains_pending_then_voids_unfinished_games(self):
        with scheduler() as league:
            original = league.learner_actions

            def learner(*args):
                actions = original(*args)
                if len(league.batches) == 2:
                    league.stop = True
                return actions

            league.learner_actions = learner
            rows, transitions = league.play(jobs(1, 5, 5, 5), object(), 'train', True)
            self.assertEqual(Counter(row['result'] for row in rows), {'W': 1, 'VOID': 3})
            self.assertEqual([row['error'] for row in rows if row['result'] == 'VOID'],
                             ['deadline_or_stop'] * 3)
            self.assertEqual(len(transitions), 1)
            self.assertEqual(league.pace['engine_steps'], 4)
            self.assertTrue(all(not client.lock.locked() for client in league.clients))

    def test_worker_error_voids_and_restarts_without_pending_request(self):
        with scheduler() as league:
            league.clients[0].fail = True
            old_client = league.clients[0]
            rows, transitions = league.play(jobs(2, 3, 3, 3), object(), 'train', True)
            self.assertEqual(Counter(row['result'] for row in rows), {'W': 3, 'VOID': 1})
            self.assertEqual(league.restarts, [0])
            self.assertTrue(old_client.closed)
            self.assertEqual(next(row['error'] for row in rows if row['worker'] == 0), 'fake_engine_error')
            self.assertEqual(len(transitions), 9)

    def test_terminal_rebalances_and_merges_singleton_tail(self):
        with scheduler() as league:
            rows, _ = league.play(jobs(1, 3, 3, 3), object(), 'train', True)
            self.assertEqual(league.batches, [(0, 1), (2, 3), (1, 2, 3), (1, 2, 3)])
            self.assertEqual(len(rows), 4)

    def test_evaluation_chunks_reuse_workers_only_after_previous_chunk(self):
        with scheduler() as league:
            rows, transitions = league.play(jobs(2, 2, 2, 2, 1, 1, 1), object(), 'selection')
            self.assertEqual(len(rows), 7)
            self.assertEqual(transitions, [])
            reset_positions = [i for i, row in enumerate(league.trace) if row[0] == 'reset']
            first_records = [i for i, row in enumerate(league.trace) if row[0] == 'record'][:4]
            self.assertGreater(reset_positions[4], max(first_records))

    def test_more_groups_than_workers_and_single_worker(self):
        for count in (1, 3):
            with self.subTest(workers=count), scheduler(count=count, groups=10) as league:
                rows, _ = league.play(jobs(*([2] * count)), object(), 'train', True)
                self.assertEqual(len(rows), count)

    def test_default_is_two_groups_for_programmatic_callers(self):
        with scheduler() as league:
            del league.args.step_groups
            league.play(jobs(1, 1, 1, 1), object(), 'train', True)
            self.assertEqual(league.batches, [(0, 1), (2, 3)])

    def test_reset_error_restarts_worker_and_other_games_complete(self):
        with scheduler() as league:
            original = league.reset

            def reset(worker, job):
                if worker == 0:
                    raise RuntimeError('fake_reset_error')
                return original(worker, job)

            league.reset = reset
            rows, _ = league.play(jobs(2, 2, 2, 2), object(), 'train', True)
            self.assertEqual(Counter(row['result'] for row in rows), {'W': 3, 'VOID': 1})
            self.assertEqual(league.restarts, [0])

    def test_group_wait_timeout_rechecks_stop_and_drains_before_return(self):
        with scheduler() as league:
            waits = []

            def wait(pending, pipeline):
                waits.append(tuple(game.worker for game in pending.values()))
                league.stop = True
                return set()  # A .5s group wait timed out while stop arrived.

            league._completed_steps = wait
            rows, _ = league.play(jobs(5, 5, 5, 5), object(), 'train', True)
            self.assertEqual(waits, [(0, 1)])
            self.assertEqual(Counter(row['result'] for row in rows), {'VOID': 4})
            self.assertEqual(league.pace['engine_steps'], 4)
            self.assertTrue(all(not client.lock.locked() for client in league.clients))


if __name__ == '__main__':
    main()
