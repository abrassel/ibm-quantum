#!/usr/bin/python3

import json
import operator
import subprocess
import sys
import re

operations = {
    "Sum": operator.add,
    "Mul": operator.mul,
    "Div": operator.truediv,
}


def evaluate_program(init_state, commands):
    cur_val = init_state
    for operation in commands:
        func = operations[operation["type"]]
        cur_val = func(cur_val, operation["value"])

    return int(cur_val)


def duplicate_output(results):
    return "\n".join(
        f"Result for id {id}: {int(result)}"
        for id, result
        in results
    ) + "\n"


def load_program(file):
    with open(file) as fp:
        programs = json.load(fp)

    if isinstance(programs, dict):
        programs = [programs]

    return programs


def unpack_result(bstring):
    raw_results = str(bstring, "utf-8")
    results = {}
    for raw_result in raw_results.splitlines():
        id, value = re.match(r"id ([\w\d-]+): (\d+)", raw_result).groups()
        results[id] = int(value)

    return results


def run(file):
    programs = load_program(file)
    results = {
        program["id"]: evaluate_program(
            program["initial_value"], program["operations"]
        )
        for program in programs
    }

    return results


def check(fname):
    result = subprocess.run(
        f"cargo run {fname} --acme http://127.0.0.1:8000 --madrid http://127.0.0.1:8001".split(),
        capture_output=True
    ).stdout
    result = unpack_result(result)
    expected = run(fname)
    assert result == expected, f"Diff: {set(result.items()) ^ set(expected.items())}"


if __name__ == "__main__":
    print(run(sys.argv[1]))
