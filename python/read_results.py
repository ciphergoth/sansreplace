# Copyright 2018 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import csv
import dataclasses
import sys

import machine
from algorithms import ALGORITHMS

@dataclasses.dataclass
class Test:
    alg: str
    n: int
    k: int
    time: float

def read_file(fname):
    with fname.open(newline='') as f:
        for row in csv.reader(f):
            if row[0] == 'result':
                yield Test(row[1], int(row[2]), int(row[3]), float(row[4]))

def setup(fname):
    "The machine setup a CSV records, as in machine.KEYS"
    with fname.open(newline='') as f:
        info = {row[1]: row[2] for row in csv.reader(f) if row[0] == 'info'}
    return tuple(info.get(k) for k in machine.KEYS)

def read(path):
    """The results in a CSV, or in a directory holding a CSV per algorithm, in
    which case warn if the CSVs were not all timed on the same setup"""
    if not path.is_dir():
        yield from read_file(path)
        return
    csvs = sorted(path.glob("*.csv"))
    setups = {}
    for fname in csvs:
        setups.setdefault(setup(fname), []).append(fname.stem)
    if len(setups) > 1:
        print(f"warning: {path} mixes results from different setups:", file=sys.stderr)
        for s, algs in setups.items():
            print(f"  {', '.join(algs)}: {dict(zip(machine.KEYS, s))}", file=sys.stderr)
    for fname in csvs:
        yield from read_file(fname)

def read_for_plot(path):
    data = {}
    for item in read(path):
        if item.k != 0:
            data.setdefault(item.n, {}).setdefault(item.alg, []).append((item.k, 1E9 * item.time/item.k))
    all_algorithms = set()
    for nv in data.values():
        for alg, d in nv.items():
            d.sort()
            all_algorithms.add(alg)
    data = list(data.items())
    data.sort()
    return sorted(all_algorithms, key=ALGORITHMS.index), data
