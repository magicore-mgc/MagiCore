"""Print all lifecycle lane and dimension verdicts into the Actions log."""

from __future__ import annotations

import json


def main() -> None:
    with open("docs/specs/lifecycleCapabilityMatrix.json") as matrix_file:
        data = json.load(matrix_file)
    print(
        f"lifecycle matrix schema_version={data.get('schema_version')} "
        f"dimensions={len(data.get('dimensions', []))}"
    )
    for lane in data["lanes"]:
        print(
            f"{lane['core']}/{lane['language']}: "
            f"verdict={lane['verdict']} native_pm={lane.get('native_pm_verdict')}"
        )
        for dimension, status in lane["dimensions"].items():
            print(f"  {dimension:<14} {status}")


if __name__ == "__main__":
    main()
