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
import attr

@attr.s(auto_attribs=True)
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
