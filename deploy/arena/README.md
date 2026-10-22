# Arena on the VPS

Public URL: **https://calgarypermit.ca/clashroyale/**

Arena runs on `vps` (`187.124.144.206`), using the existing Calgary Permit
certificate and an additional `/clashroyale/` Nginx location. The existing
Calgary Permit website remains at `/`. The Lab, replay collector, research data
and training jobs are not deployed by this runtime package.

## Runtime

- Service: `clash-arena.service`, enabled at boot, restart on failure.
- User: `clasharena`, without an interactive shell.
- Code: `/opt/clash-arena/current`, a symlink into `releases/`.
- Python environment: `/opt/clash-arena/venv/` (CPU-only PyTorch).
- Writable match/cache data: `/var/lib/clash-arena/`.
- Environment: `/etc/clash-arena.env` (root-only; contains the session secret).
- Backend: Uvicorn/Starlette, **one worker**, `127.0.0.1:8800` only.
- HTTPS proxy: `/etc/nginx/snippets/clash-arena.conf`, included in
  `/etc/nginx/sites-available/calgary-permit-frontend`.
- Create-request rate zone: `/etc/nginx/conf.d/clash-arena-rate.conf`.

Each browser gets a signed Secure/HttpOnly cookie scoped to `/clashroyale/`.
It can list, view and control only its own matches. Cross-origin writes are
rejected. There is one active match slot globally, with a per-browser cap of
one and a 15-minute maximum session lifetime, including pauses. When occupied,
new starts return 429; players can retry after the slot is released. Unwatched
matches retain the two-minute idle timeout. Start attempts are rate limited.

Rendering uses the existing original-asset Rust Painter, targeting up to 60 FPS
with JPEG quality 88 on this VPS. Actual frame rate depends on available CPU.
SIMD libjpeg-turbo encodes RGBA directly in Rust; Python forwards compressed
bytes through a persistent MJPEG connection. A slow viewer receives the latest
frame without accumulating a render queue. Idle frames repeat every five seconds
to keep paused streams connected. Local defaults remain 30 FPS / quality 88. The service
has a 150% CPU quota and a 4 GiB memory limit; it cannot consume all VPS RAM.
No GPU or fast training engine is needed.

Release `20261022-03` was checked over public HTTPS: 16.81 delivered FPS in a
15.4-second sample after warmup, with 69.4% host CPU steal measured during
startup. The local renderer reached 59.58 FPS; median JPEG encode time was
1.34 ms. This is a measured sample, not a guaranteed rate. Mouse/touch play,
pause/resume, reset, fallback rendering and session isolation passed, alongside
52 web tests. [Verification record](verification.json).

## Operations

```sh
ssh root@187.124.144.206 'systemctl status clash-arena --no-pager'
ssh root@187.124.144.206 'journalctl -u clash-arena -f'
ssh root@187.124.144.206 'systemctl restart clash-arena'
curl -fsS https://calgarypermit.ca/clashroyale/api/health
```

Restarting ends current matches. The match manager is intentionally in one
process; increasing Uvicorn workers would create independent slot counters.

## Packaging and updating

Build the native watcher and original renderer locally, then package:

```sh
cargo build --release --offline --manifest-path engine/native-engine/Cargo.toml --bin native-watch
TURBOJPEG_STATIC=1 cargo build --offline --manifest-path engine/native-spectator/Cargo.toml --features web-jpeg --bin clash-web-renderer
.venv/bin/python deploy/arena/package.py /tmp/clash-arena-runtime.tar.gz
```

The browser renderer build needs CMake, a C compiler and NASM for SIMD. Its
JPEG library is statically linked, so the VPS needs no codec installation.
The `web-jpeg` feature is separate from the desktop viewer build.

The allowlist includes Arena/shared Python code, inference adapters, card
metadata/art, four controller entries and their three saved checkpoints, the
base BC checkpoint, pinned Unicorn library and original engine library, recovered engine state and original
spectator assets. It excludes Git metadata, local AGENTS instructions,
credentials, the Lab and bulk datasets. Every runtime file has a checksum in
`arena-runtime-manifest.json`.

Transfer into `/opt/clash-arena/`, verify the printed archive SHA-256, extract
into a fresh `releases/<version>/` folder and verify every file against the
manifest. Code belongs to root with group `clasharena`, readable/executable by
that group; only `/var/lib/clash-arena/` is writable by the service. Atomically
switch `current` and restart the service after verifying the new release.
The previous release can be restored by switching the symlink back.

The checkpoint’s Unicorn host path and the original engine library path are
relocated using `CLASH_ENGINE_ROOT`; their pinned hashes remain checked. Preserve
the library and checkpoint bytes. Existing cache files are valid only for their
pinned binary/checkpoint identity.

Production Python packages installed in the VPS environment:

```text
torch==2.13.0+cpu  (https://download.pytorch.org/whl/cpu)
numpy==2.5.1
Pillow==12.3.0
lxml==6.1.1
gymnasium==1.4.0
starlette==1.9.0
uvicorn==0.38.0
itsdangerous==2.2.0
```

Run `nginx -t` before reloading Nginx. An original Calgary frontend config
backup was retained beside its configuration before adding Arena’s include.

## Preferred domain later

`clashroyale.kemongo.com` requires an A record pointing at this VPS. It was
not changed: Chrome was unavailable and the user selected the Calgary fallback.
After DNS exists, add its Nginx host/certificate and set `ARENA_BASE_PATH` to
empty and `ARENA_ALLOWED_HOSTS` to the new hostname, then restart Arena.
