"""Run the Arena website: ``.venv/bin/python -m webapps.arena``."""
from __future__ import annotations

import argparse


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host", default="127.0.0.1",
                        help="use 0.0.0.0 to allow other devices on your network (no authentication)")
    parser.add_argument("--port", type=int, default=8800)
    parser.add_argument("--max-matches", type=int, default=3,
                        help="each match runs its own engine process (~0.5 GB RAM)")
    args = parser.parse_args()
    from .server import app, manager, warmup
    manager.max_matches = args.max_matches
    try:
        warmup()
        app.serve(args.host, args.port)
    finally:
        manager.close_all()


if __name__ == "__main__":
    main()
