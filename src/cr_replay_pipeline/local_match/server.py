"""Local web control: pick both AIs and both decks, then start a battle."""

from __future__ import annotations

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlparse

from .agents import build_agent, list_agents
from .catalog import load_catalog, repo_root
from .engine import REAL_ENGINE_BLOCKER
from .presets import cards_for_preset, preset_payload
from .prepare import default_var_dir
from .runner import progress_table, run_match
from .wire import LOOPBACK_HOSTS, ProtocolClient, ProtocolServer, RemoteHostRefused

UI_PATH = Path(__file__).with_name("ui.html")
MAX_BODY = 256 * 1024


class MatchControl:
    def __init__(self, catalog, models_root: Path, device: str | None = None):
        self.catalog = catalog
        self.models_root = models_root
        self.device = device
        self._lock = threading.Lock()
        self._stop = threading.Event()
        self._thread: threading.Thread | None = None
        self._protocol: ProtocolServer | None = None
        self.snapshot = self._idle()

    def _idle(self) -> dict:
        return {
            "running": False,
            "finished": False,
            "phase": "idle",
            "work": 0,
            "actions": 0,
            "sim_s": 0.0,
            "sim_left_s": None,
            "eta_wall_s": None,
            "max_seconds": None,
            "engine": "control-harness",
            "real_engine": False,
            "combat": False,
            "transport": None,
            "protocol": "cr-local-match/1",
            "blocker": REAL_ENGINE_BLOCKER,
            "opened_in": None,
            "menu_steps": 0,
            "red": None,
            "blue": None,
            "red_deck": None,
            "blue_deck": None,
            "events": [],
            "lines": [],
            "last": None,
            "result": None,
            "error": None,
        }

    def view(self) -> dict:
        with self._lock:
            return dict(self.snapshot)

    def update(self, view: dict) -> None:
        with self._lock:
            self.snapshot = dict(view)

    def stop(self) -> dict:
        self._stop.set()
        thread = self._thread
        if thread is not None:
            thread.join(timeout=5)
        return self.view()

    def launch(
        self,
        *,
        red_agent: str,
        blue_agent: str,
        red: list[str],
        blue: list[str],
        seed: int,
        max_seconds: float,
        speed: float,
        endpoint: str | None,
    ) -> dict:
        max_seconds = min(600.0, max(1.0, float(max_seconds)))
        speed = min(8.0, max(0.0, float(speed)))
        agents_index = {row["id"]: row for row in list_agents(self.models_root)}
        for label, agent_id in (("red", red_agent), ("blue", blue_agent)):
            row = agents_index.get(agent_id)
            if row is None or not row["loadable"]:
                detail = row["detail"] if row else "unknown agent"
                raise ValueError(f"{label} agent {agent_id} is not loadable: {detail}")
        self.catalog.validate_deck(red)
        self.catalog.validate_deck(blue)
        with self._lock:
            if self.snapshot.get("running"):
                raise RuntimeError("a match is already running")
            self._stop = threading.Event()
            self.snapshot = {
                **self._idle(),
                "running": True,
                "phase": "battle",
                "opened_in": "battle",
                "max_seconds": max_seconds,
                "sim_left_s": max_seconds,
                "red_deck": list(red),
                "blue_deck": list(blue),
                "red_agent": red_agent,
                "blue_agent": blue_agent,
                "last": "starting",
            }

        def target() -> None:
            protocol = ProtocolServer(self.catalog, seed=seed)
            self._protocol = protocol
            try:
                if endpoint:
                    client = ProtocolClient(endpoint)
                else:
                    client = ProtocolClient(protocol.start())
                agents = {
                    "red": build_agent(
                        red_agent, seed=seed, root=self.models_root, device=self.device
                    ),
                    "blue": build_agent(
                        blue_agent, seed=seed + 1, root=self.models_root, device=self.device
                    ),
                }
                def on_update(view: dict) -> None:
                    view["red_agent"] = red_agent
                    view["blue_agent"] = blue_agent
                    self.update(view)

                run_match(
                    client,
                    agents,
                    red=red,
                    blue=blue,
                    seed=seed,
                    max_seconds=max_seconds,
                    speed=speed,
                    stop_event=self._stop,
                    on_update=on_update,
                    echo=print,
                )
            except (RemoteHostRefused, OSError, RuntimeError, ValueError, FileNotFoundError) as exc:
                failed = self.view()
                failed["running"] = False
                failed["finished"] = True
                failed["error"] = str(exc)
                failed["phase"] = "error"
                self.update(failed)
                print(f"local-match error: {exc}", flush=True)
            finally:
                protocol.stop()
                self._protocol = None

        self._thread = threading.Thread(target=target, name="local-match", daemon=True)
        self._thread.start()
        return self.view()


def _deck_from_request(payload: dict, side: str) -> list[str]:
    cards = payload.get(f"{side}_cards")
    if isinstance(cards, list) and cards:
        return [str(card) for card in cards]
    preset = payload.get(f"{side}_preset")
    if preset:
        return cards_for_preset(str(preset))
    raise ValueError(f"choose {side} cards or a {side}_preset")


def make_handler(control: MatchControl, prepare_status: Path):
    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, fmt: str, *args) -> None:
            return

        def _send(self, payload, status: int = 200, content_type: str = "application/json") -> None:
            if isinstance(payload, str):
                encoded = payload.encode("utf-8")
            else:
                encoded = json.dumps(payload).encode("utf-8")
            self.send_response(status)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(encoded)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(encoded)

        def _read_json(self) -> dict:
            length = int(self.headers.get("Content-Length", "0") or 0)
            if length > MAX_BODY:
                raise ValueError("body too large")
            if length == 0:
                return {}
            return json.loads(self.rfile.read(length).decode("utf-8"))

        def do_GET(self) -> None:
            path = urlparse(self.path).path
            if path == "/favicon.ico":
                self.send_response(204)
                self.send_header("Content-Length", "0")
                self.end_headers()
                return
            if path in ("/", "/index.html"):
                self._send(UI_PATH.read_text(encoding="utf-8"), content_type="text/html; charset=utf-8")
                return
            if path == "/api/catalog":
                self._send(
                    {
                        "cards": [card.public() for card in control.catalog.cards],
                        "presets": preset_payload(),
                    }
                )
                return
            if path == "/api/agents":
                self._send({"agents": list_agents(control.models_root)})
                return
            if path == "/api/status":
                prepared = None
                if prepare_status.is_file():
                    try:
                        prepared = json.loads(prepare_status.read_text(encoding="utf-8"))
                    except json.JSONDecodeError:
                        prepared = None
                self._send(
                    {
                        "ok": True,
                        "protocol": "cr-local-match/1",
                        "engine": "control-harness",
                        "real_engine": False,
                        "blocker": REAL_ENGINE_BLOCKER,
                        "prepare": prepared,
                        "watch": "curl -s http://127.0.0.1:8770/api/match/table",
                    }
                )
                return
            if path == "/api/match":
                self._send(control.view())
                return
            if path == "/api/match/table":
                view = control.view()
                eta = view.get("eta_wall_s")
                left = view.get("sim_left_s")
                text = progress_table(
                    int(view.get("work") or 0),
                    float(view.get("sim_s") or 0),
                    float(left if left is not None else 0),
                    None if eta is None else float(eta),
                    str(view.get("phase") or "idle"),
                    str(view.get("last") or ""),
                )
                self._send(text, content_type="text/plain; charset=utf-8")
                return
            self._send({"ok": False, "error": "not found"}, 404)

        def do_POST(self) -> None:
            path = urlparse(self.path).path
            try:
                payload = self._read_json()
            except (UnicodeError, json.JSONDecodeError, ValueError) as exc:
                self._send({"ok": False, "error": str(exc)}, 400)
                return
            if path == "/api/match/stop":
                self._send(control.stop())
                return
            if path != "/api/match":
                self._send({"ok": False, "error": "not found"}, 404)
                return
            try:
                red = _deck_from_request(payload, "red")
                blue = _deck_from_request(payload, "blue")
                view = control.launch(
                    red_agent=str(payload.get("red_agent") or "random"),
                    blue_agent=str(payload.get("blue_agent") or "random"),
                    red=red,
                    blue=blue,
                    seed=int(payload.get("seed") or 1),
                    max_seconds=float(payload.get("max_seconds") or 180),
                    speed=float(payload.get("speed") or 0),
                    endpoint=payload.get("endpoint") or None,
                )
            except (RuntimeError, ValueError, RemoteHostRefused) as exc:
                self._send({"ok": False, "error": str(exc)}, 400)
                return
            self._send(view)

    return Handler


def serve_local_match(
    host: str = "127.0.0.1",
    port: int = 8770,
    *,
    device: str | None = None,
    models_root: Path | None = None,
    launch: dict | None = None,
) -> None:
    if host not in LOOPBACK_HOSTS:
        raise RuntimeError("local-match only binds to a loopback address")
    catalog = load_catalog()
    control = MatchControl(catalog, Path(models_root) if models_root else repo_root(), device)
    handler = make_handler(control, default_var_dir() / "prepare-status.json")
    server = ThreadingHTTPServer((host, port), handler)
    bound_host, bound_port = server.server_address[:2]
    url = f"http://{bound_host}:{bound_port}"
    print(f"Local match control  {url}", flush=True)
    print("work  sim_s  left_s  eta_s are on /api/match/table", flush=True)
    print(f"Stream: watch -n 1 'curl -s {url}/api/match/table'", flush=True)
    if launch:
        control.launch(**launch)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nstopping", flush=True)
    finally:
        control.stop()
        server.shutdown()
        server.server_close()


def run_headless(
    *,
    red_agent: str,
    blue_agent: str,
    red: list[str],
    blue: list[str],
    seed: int = 1,
    max_seconds: float = 30,
    speed: float = 0,
    device: str | None = None,
    endpoint: str | None = None,
) -> dict:
    catalog = load_catalog()
    catalog.validate_deck(red)
    catalog.validate_deck(blue)
    max_seconds = min(600.0, max(1.0, float(max_seconds)))
    speed = min(8.0, max(0.0, float(speed)))
    root = repo_root()
    protocol = ProtocolServer(catalog, seed=seed)
    try:
        client = ProtocolClient(endpoint) if endpoint else ProtocolClient(protocol.start())
        agents = {
            "red": build_agent(red_agent, seed=seed, root=root, device=device),
            "blue": build_agent(blue_agent, seed=seed + 1, root=root, device=device),
        }
        return run_match(
            client,
            agents,
            red=red,
            blue=blue,
            seed=seed,
            max_seconds=max_seconds,
            speed=speed,
        )
    finally:
        protocol.stop()
