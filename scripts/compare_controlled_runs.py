"""Paired battle bootstrap; evidence for manual gates, never auto-promote."""

import argparse
import json
from pathlib import Path
from cr_replay_pipeline.controlled_experiment import compare_run_dirs


def compare(left, right):
    return compare_run_dirs(left, right)


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("baseline", type=Path)
    p.add_argument("candidate", type=Path)
    p.add_argument("--output", type=Path, required=True)
    a = p.parse_args()
    result = compare(a.baseline, a.candidate)
    a.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))
