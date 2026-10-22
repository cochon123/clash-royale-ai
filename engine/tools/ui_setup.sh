#!/usr/bin/env bash
set -euo pipefail
project_dir="$(cd "$(dirname "$0")/.." && pwd)"
python3 -m venv "$project_dir/.venv-ui"
"$project_dir/.venv-ui/bin/pip" install 'sc5-parser @ git+https://github.com/obus-globus/sc5-parser.git@9108080256a0df6cd772d81afdbc26047d79d78d'
printf '%s\n' 'Run .venv-ui/bin/python tools/ui_recover.py from the project directory.'
