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

@dataclasses.dataclass
class Test:
    alg: str
    n: int
    k: int
    time: float

def read(fname):
    with fname.open(newline='') as f:
        for row in csv.reader(f):
            if row[0] == 'result':
                yield Test(row[1], int(row[2]), int(row[3]), float(row[4]))

def read_for_plot(fname):
    data = {}
    for item in read(fname):
        if item.k != 0:
            data.setdefault(item.n, {}).setdefault(item.alg, []).append((item.k, 1E9 * item.time/item.k))
    all_algorithms = set()
    for nv in data.values():
        for alg, d in nv.items():
            d.sort()
            all_algorithms.add(alg)
    data = list(data.items())
    data.sort()
    return list(sorted(all_algorithms)), data
