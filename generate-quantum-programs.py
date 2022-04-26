#  The MIT License (MIT)
# Copyright © 2022 IBM Quantum
# Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
# The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
# THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

import uuid
import random
from dataclasses import dataclass
from enum import IntEnum
import sys
import getopt
import json
from json import JSONEncoder
from typing import List

from numpy import isin


def usage():
    print(
        f"""
This is a Quantum Program random json generator for the Quauntum Systems hiring test 2022.
Usage: {sys.argv[0]} [OPTIONS]
OPTIONS:
-n <number of random arithmetic operations per program> Defaults to 3
-m <number of programs> Defaults to 1
    """
    )
    sys.exit()


class ArithmeticOperation(IntEnum):
    Sum = 1
    Mul = 2
    Div = 3
    InitState = 4


class ControlInstrument(IntEnum):
    Acme = 1
    Madrid = 2


@dataclass
class Operation:
    type: ArithmeticOperation
    value: int


@dataclass
class QuantumProgram:
    id: str
    control_instrument: ControlInstrument
    initial_value: int
    operations: List[Operation]


class MyEncoder(JSONEncoder):
    def default(self, o):
        if isinstance(o, Operation):
            return {"type": o.type.name, "value": o.value}

        if isinstance(o, QuantumProgram):
            o.control_instrument = o.control_instrument.name

        return o.__dict__


def generate_quantum_programs(number_of_operations, number_of_programs):
    quantum_programs = []
    for i in range(number_of_programs):
        arithmetic_opers = []
        for j in range(number_of_operations):
            arithmetic_opers.append(
                Operation(
                    type=ArithmeticOperation(random.randint(1, 3)),
                    value=random.randint(1, 10),
                )
            )
        quantum_programs.append(
            QuantumProgram(
                id=str(uuid.uuid4()),
                control_instrument=ControlInstrument.Acme,
                initial_value=random.randint(0, 10),
                operations=arithmetic_opers,
            )
        )

    return quantum_programs


def to_json(quantum_programs):
    return json.dumps(quantum_programs, cls=MyEncoder)


def main():
    argv = sys.argv[1:]
    options, args = getopt.getopt(argv, "hn:m:")

    number_of_operations = 3
    number_of_programs = 1

    for opt, arg in options:
        if opt in ["-h", "--help"]:
            usage()

        if opt == "-n":
            number_of_operations = int(arg)

        if opt == "-m":
            number_of_programs = int(arg)

    quantum_programs = generate_quantum_programs(number_of_operations, number_of_programs)
    quantum_programs_json = to_json(quantum_programs)

    print(quantum_programs_json)


if __name__ == "__main__":
    main()
