from __future__ import annotations

import json
import sqlite3
import threading
import time
from contextlib import contextmanager
from pathlib import Path
from collections.abc import Iterator
from urllib.parse import parse_qs, unquote, urlparse


def normalize_tag(value: str) -> str:
    return value.strip().upper().lstrip("#").replace("O", "0")


def recommended_workers(interval_seconds: float) -> int:
    """Two pages already keep the globally paced replay endpoint saturated."""
    del interval_seconds
    return 2


def replay_participants(record: dict) -> tuple[str | None, set[str]]:
    """Return the browsed player and every participant encoded in a replay URL."""
    source = None
    referrer = record.get("referrer_url") or ""
    parts = urlparse(referrer).path.strip("/").split("/")
    if len(parts) >= 2 and parts[0] == "player":
        source = normalize_tag(unquote(parts[1]))

    query = parse_qs(urlparse(record.get("request_url") or "").query)
    participants: set[str] = set()
    for key in ("team_tags", "opponent_tags"):
        for group in query.get(key, []):
            for value in group.split(","):
                tag = normalize_tag(value)
                if tag:
                    participants.add(tag)
    return source, participants


class Frontier:
    """Durable player crawl frontier and globally shared request governor."""

    def __init__(
        self,
        path: str | Path,
        *,
        initial_interval: float = 2.2,
        minimum_interval: float = 0.75,
        calibration_window: int = 40,
    ) -> None:
        self.path = Path(path)
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self.initial_interval = initial_interval
        self.minimum_interval = minimum_interval
        self.calibration_window = calibration_window
        self._init_lock = threading.Lock()
        self._initialize()

    @contextmanager
    def _connect(self) -> Iterator[sqlite3.Connection]:
        connection = sqlite3.connect(self.path, timeout=30)
        connection.row_factory = sqlite3.Row
        connection.execute("PRAGMA busy_timeout=30000")
        try:
            yield connection
            connection.commit()
        except BaseException:
            connection.rollback()
            raise
        finally:
            connection.close()

    def _initialize(self) -> None:
        with self._init_lock, self._connect() as db:
            db.execute("PRAGMA journal_mode=WAL")
            db.executescript(
                """
                CREATE TABLE IF NOT EXISTS players (
                    tag TEXT PRIMARY KEY,
                    status TEXT NOT NULL DEFAULT 'queued',
                    priority INTEGER NOT NULL DEFAULT 50,
                    depth INTEGER NOT NULL DEFAULT 0,
                    source TEXT NOT NULL DEFAULT 'unknown',
                    attempts INTEGER NOT NULL DEFAULT 0,
                    next_eligible REAL NOT NULL DEFAULT 0,
                    lease_until REAL,
                    worker_id TEXT,
                    last_error TEXT,
                    discovered_at REAL NOT NULL,
                    updated_at REAL NOT NULL,
                    last_completed_at REAL
                );
                CREATE INDEX IF NOT EXISTS players_claim_idx
                    ON players(status, next_eligible, priority DESC, depth, discovered_at);
                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );
                """
            )
            defaults = {
                "paused": False,
                "pause_reason": "",
                "interval_seconds": self.initial_interval,
                "clean_streak": 0,
                "next_request_at": 0.0,
                "blocked_until": 0.0,
                "total_rate_limits": 0,
                "recent_rate_results": [],
                "rate_controller_version": 3,
            }
            db.executemany(
                "INSERT OR IGNORE INTO settings(key, value) VALUES (?, ?)",
                ((key, json.dumps(value)) for key, value in defaults.items()),
            )
            # Do not carry a heavily inflated v1 interval into the gentler
            # rolling-window controller.
            if int(self._get(db, "rate_controller_version") or 1) < 3:
                interval = float(self._get(db, "interval_seconds"))
                self._set(db, "interval_seconds", min(interval, self.initial_interval * 1.5))
                self._set(db, "clean_streak", 0)
                self._set(db, "recent_rate_results", [])
                self._set(db, "rate_controller_version", 3)

    @staticmethod
    def _get(db: sqlite3.Connection, key: str):
        row = db.execute("SELECT value FROM settings WHERE key = ?", (key,)).fetchone()
        return json.loads(row[0]) if row else None

    @staticmethod
    def _set(db: sqlite3.Connection, key: str, value) -> None:
        db.execute(
            "INSERT INTO settings(key, value) VALUES (?, ?) "
            "ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            (key, json.dumps(value)),
        )

    def seed(
        self,
        tags: list[str] | set[str],
        *,
        priority: int = 100,
        depth: int = 0,
        source: str = "seed",
    ) -> int:
        now = time.time()
        normalized = {normalize_tag(tag) for tag in tags if normalize_tag(tag)}
        with self._connect() as db:
            before = db.total_changes
            db.executemany(
                """INSERT OR IGNORE INTO players
                   (tag, status, priority, depth, source, discovered_at, updated_at)
                   VALUES (?, 'queued', ?, ?, ?, ?, ?)""",
                ((tag, priority, depth, source, now, now) for tag in normalized),
            )
            return db.total_changes - before

    def bootstrap(self, raw_dir: str | Path) -> dict:
        records = 0
        sources: set[str] = set()
        participants: set[str] = set()
        for path in Path(raw_dir).glob("*.json"):
            try:
                record = json.loads(path.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError):
                continue
            source, found = replay_participants(record)
            records += 1
            participants.update(found)
            if source:
                sources.add(source)

        now = time.time()
        with self._connect() as db:
            before = db.total_changes
            db.executemany(
                """INSERT OR IGNORE INTO players
                   (tag, status, priority, depth, source, discovered_at, updated_at)
                   VALUES (?, 'queued', 80, 1, 'replay-bootstrap', ?, ?)""",
                ((tag, now, now) for tag in participants - sources),
            )
            queued = db.total_changes - before
            for tag in sources:
                db.execute(
                    """INSERT OR IGNORE INTO players
                       (tag, status, priority, depth, source, discovered_at, updated_at, last_completed_at)
                       VALUES (?, 'completed', 100, 0, 'replay-bootstrap', ?, ?, ?)""",
                    (tag, now, now, now),
                )
        return {
            "records": records,
            "sources_completed": len(sources),
            "participants": len(participants),
            "new_queued": queued,
        }

    def discover(self, source_tag: str | None, tags: set[str]) -> int:
        depth = 0
        if source_tag:
            with self._connect() as db:
                row = db.execute(
                    "SELECT depth FROM players WHERE tag = ?", (normalize_tag(source_tag),)
                ).fetchone()
                depth = int(row[0]) + 1 if row else 1
        return self.seed(tags, priority=max(10, 80 - depth), depth=depth, source="opponent")

    def claim(self, worker_id: str, lease_seconds: int = 600) -> dict | None:
        now = time.time()
        with self._connect() as db:
            db.execute("BEGIN IMMEDIATE")
            if self._get(db, "paused"):
                return None
            db.execute(
                """UPDATE players SET status='queued', worker_id=NULL, lease_until=NULL,
                   updated_at=?, last_error='expired lease'
                   WHERE status='leased' AND lease_until < ?""",
                (now, now),
            )
            existing = db.execute(
                """SELECT tag, depth, attempts FROM players
                   WHERE status='leased' AND worker_id=? AND lease_until >= ?
                   ORDER BY updated_at DESC LIMIT 1""",
                (worker_id, now),
            ).fetchone()
            if existing is not None:
                return {
                    "tag": existing["tag"],
                    "depth": existing["depth"],
                    "attempt": existing["attempts"],
                }
            row = db.execute(
                """SELECT tag, depth, attempts FROM players
                   WHERE status='queued' AND next_eligible <= ?
                   ORDER BY priority DESC, depth, discovered_at LIMIT 1""",
                (now,),
            ).fetchone()
            if row is None:
                return None
            db.execute(
                """UPDATE players SET status='leased', worker_id=?, lease_until=?,
                   attempts=attempts+1, updated_at=? WHERE tag=?""",
                (worker_id, now + lease_seconds, now, row["tag"]),
            )
            return {"tag": row["tag"], "depth": row["depth"], "attempt": row["attempts"] + 1}

    def complete(self, tag: str, *, retry: bool = False, error: str | None = None) -> str:
        tag = normalize_tag(tag)
        now = time.time()
        with self._connect() as db:
            row = db.execute("SELECT attempts FROM players WHERE tag=?", (tag,)).fetchone()
            if row is None:
                return "unknown"
            attempts = int(row[0])
            if retry and attempts < 3:
                status = "queued"
                next_eligible = now + min(900, 30 * (2 ** max(0, attempts - 1)))
            elif retry:
                status = "manual"
                next_eligible = 0
            else:
                status = "completed"
                next_eligible = 0
            db.execute(
                """UPDATE players SET status=?, next_eligible=?, lease_until=NULL,
                   worker_id=NULL, last_error=?, updated_at=?,
                   last_completed_at=CASE WHEN ?='completed' THEN ? ELSE last_completed_at END
                   WHERE tag=?""",
                (status, next_eligible, error, now, status, now, tag),
            )
            return status

    def pause(self, reason: str = "paused by operator") -> None:
        with self._connect() as db:
            self._set(db, "paused", True)
            self._set(db, "pause_reason", reason)

    def resume(self) -> None:
        with self._connect() as db:
            self._set(db, "paused", False)
            self._set(db, "pause_reason", "")

    def rate_lease(self, worker_id: str = "worker") -> dict:
        del worker_id  # reserved for per-worker diagnostics
        now = time.time()
        with self._connect() as db:
            db.execute("BEGIN IMMEDIATE")
            interval = float(self._get(db, "interval_seconds"))
            next_at = float(self._get(db, "next_request_at"))
            blocked = float(self._get(db, "blocked_until"))
            granted_at = max(now, next_at, blocked)
            self._set(db, "next_request_at", granted_at + interval)
            return {
                "wait_ms": max(0, round((granted_at - now) * 1000)),
                "interval_ms": round(interval * 1000),
                "blocked_until": blocked,
            }

    def rate_result(self, *, rate_limited: bool, retry_after: float = 30) -> None:
        now = time.time()
        with self._connect() as db:
            db.execute("BEGIN IMMEDIATE")
            interval = float(self._get(db, "interval_seconds"))
            recent = list(self._get(db, "recent_rate_results") or [])
            recent.append(1 if rate_limited else 0)
            recent = recent[-200:]
            self._set(db, "recent_rate_results", recent)
            if rate_limited:
                interval = max(self.initial_interval, interval * 1.25)
                self._set(db, "interval_seconds", interval)
                self._set(db, "clean_streak", 0)
                self._set(db, "blocked_until", now + max(1, retry_after))
                self._set(
                    db,
                    "total_rate_limits",
                    int(self._get(db, "total_rate_limits")) + 1,
                )
                return
            streak = int(self._get(db, "clean_streak")) + 1
            if streak >= self.calibration_window:
                interval = max(self.minimum_interval, interval * 0.93)
                streak = 0
                self._set(db, "interval_seconds", interval)
            self._set(db, "clean_streak", streak)

    def status(self) -> dict:
        now = time.time()
        with self._connect() as db:
            counts = {
                row["status"]: row["count"]
                for row in db.execute(
                    "SELECT status, COUNT(*) AS count FROM players GROUP BY status"
                )
            }
            active = [
                dict(row)
                for row in db.execute(
                    """SELECT worker_id, tag, lease_until FROM players
                       WHERE status='leased' ORDER BY worker_id"""
                )
            ]
            paused = bool(self._get(db, "paused"))
            interval = float(self._get(db, "interval_seconds"))
            recent = list(self._get(db, "recent_rate_results") or [])
            return {
                "paused": paused,
                "pause_reason": self._get(db, "pause_reason"),
                "players": {
                    "total": sum(counts.values()),
                    "queued": counts.get("queued", 0),
                    "leased": counts.get("leased", 0),
                    "completed": counts.get("completed", 0),
                    "manual": counts.get("manual", 0),
                },
                "active": active,
                "rate": {
                    "interval_seconds": round(interval, 3),
                    "recommended_workers": 0 if paused else recommended_workers(interval),
                    "clean_streak": int(self._get(db, "clean_streak")),
                    "blocked_for_seconds": round(max(0, float(self._get(db, "blocked_until")) - now), 1),
                    "total_rate_limits": int(self._get(db, "total_rate_limits")),
                    "recent_rate_limits": sum(recent),
                    "recent_requests": len(recent),
                    "recent_rate_limit_percent": round(
                        100 * sum(recent) / len(recent), 2
                    ) if recent else 0.0,
                },
            }
