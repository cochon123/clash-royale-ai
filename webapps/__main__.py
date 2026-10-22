"""Start Arena (:8800) and Lab (:8801) together from the repository root."""
from __future__ import annotations

import argparse
import signal
import subprocess
import sys
import time

from .common.paths import REPO


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host", default="127.0.0.1",
                        help="use 0.0.0.0 for other devices on your trusted network")
    parser.add_argument("--max-matches", type=int, default=3)
    args = parser.parse_args()
    children = []
    result = 0
    try:
        for module, extra in (("webapps.arena", ["--max-matches", str(args.max_matches)]),
                              ("webapps.lab", [])):
            children.append(subprocess.Popen([sys.executable, "-m", module, "--host", args.host, *extra],
                                             cwd=REPO, start_new_session=True))
        while all(child.poll() is None for child in children):
            time.sleep(0.25)
        result = next((child.returncode for child in children if child.returncode), 0)
    except KeyboardInterrupt:
        pass
    finally:
        for child in children:
            if child.poll() is None:
                child.send_signal(signal.SIGINT)
        for child in children:
            try:
                child.wait(timeout=10)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=5)
    return result


if __name__ == "__main__":
    raise SystemExit(main())
