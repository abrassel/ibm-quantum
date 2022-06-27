#!/usr/bin/python3

from pytests.utils import check
from generate_quantum_programs import to_json, generate_quantum_programs
import tempfile
import sys
import argparse


def test(new_prog):
    with tempfile.NamedTemporaryFile("w+") as json_file:
        print(new_prog)
        json_file.write(new_prog)
        json_file.flush()
        check(json_file.name)


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-n", "--number", help="Pick number of progs", type=int, default=1)
    parser.add_argument(
        "-s", "--steps", help="Number of lines of code", type=int, default=5)
    # should be mutually exclusive
    parser.add_argument("--json", help="Input to prog")
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    if args.json:
        test(args.json)
    else:
        while True:
            test(to_json(generate_quantum_programs(args.steps, args.number)))
