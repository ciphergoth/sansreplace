# Copyright 2026 Paul Crowley
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

# The setup a benchmark runs on. Each algorithm is timed in its own run, so
# results from different runs are only comparable if these all match.

# Format a time in seconds with a unit, to three significant figures.

UNITS = [(1, "s"), (1E-3, "ms"), (1E-6, "µs"), (1E-9, "ns")]

def duration(x):
    "A time in seconds, such as 1.68E-4, as a string such as 168µs"
    x = float(f"{x:.3g}")
    for scale, unit in UNITS:
        if abs(x) >= scale or scale == UNITS[-1][0]:
            break
    v = x / scale
    places = 2 if abs(v) < 10 else 1 if abs(v) < 100 else 0
    return f"{v:.{places}f}{unit}"
