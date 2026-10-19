#!/usr/bin/env sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
uv venv --allow-existing tools/reverse-env
uv pip install --python tools/reverse-env/bin/python \
  pyelftools==0.33 capstone==5.0.9 unicorn==2.1.4 lief==1.0.0
