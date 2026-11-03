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

"""The measure the results compare algorithms by: the worst time per number
output, over every n, in each decade of k.

An algorithm's score in a band is the most its time per call, divided by k,
reaches at any point with k in the band. A set of algorithms, choosing the
fastest of them at each point, is scored the same way. The band's floor is the
score of choosing the fastest of all the algorithms at every point, which no
algorithm or set can beat."""

import math

import read_results
from duration import duration

# One band per decade of k from 10; below that, per-call overhead swamps
# everything. The last band runs to the largest k timed.
EDGES = [10, 100, 1000, 10**4, 10**5, 10**6, 10**7, math.inf]
BANDS = list(zip(EDGES, EDGES[1:]))

# Scores within this factor of the best count as tied, as within the noise
# between runs
TIE = 1.03

# The algorithms that need floating point
FLOATING_POINT = {"vitterd", "hiddenshuffle"}

LATEX_NAMES = {
    "quadraticreject": "quadratic rejection",
    "rejectionsample": "set-based rejection",
    "ranksb": "RANKSB",
    "floydf2": "Floyd's F2",
    "quadraticf2": "quadratic F2",
    "iterativechoose": "iterative choosing",
    "reservoirsample": "reservoir sampling",
    "vitterd": "Vitter's D",
    "hiddenshuffle": "Hidden Shuffle",
    "select": "SELECT",
    "hsel": "HSEL",
    "cardchoose": "Cardchoose",
}

def read_times(case):
    "The time of each algorithm at each point with k >= 10, by (n, k)"
    times = {}
    for item in read_results.read(case):
        if item.k >= 10:
            times.setdefault((item.n, item.k), {})[item.alg] = item.time
    return times

def in_band(times, band):
    lo, hi = band
    return {nk: t for nk, t in times.items() if lo <= nk[1] < hi}

def score(times, algs):
    """The worst time per number output of the fastest of algs at each point,
    or infinity if none of them was timed at some point"""
    return max(min(t.get(a, math.inf) for a in algs) / k for (n, k), t in times.items())

def floor(times):
    "The score of the fastest of all the algorithms at each point"
    return max(min(t.values()) / k for (n, k), t in times.items())

def untimed(times, alg):
    "The smallest k at which alg was not timed, or None"
    return min((k for (n, k), t in times.items() if alg not in t), default=None)

def against_floor(times, algs):
    "The most the set's score exceeds the floor, as a ratio, over the bands"
    return max(score(b, algs) / floor(b) for b in (in_band(times, band) for band in BANDS))

def power(x, latex=False):
    "x with thousands separators, or in standard form from 10^4"
    if x < 10**4:
        return f"{x:,}"
    mantissa, e = f"{x:.1e}".split("e")
    e = int(e)
    if mantissa == "1.0":
        return f"\\(10^{{{e}}}\\)" if latex else f"10<sup>{e}</sup>"
    return f"\\({mantissa} \\times 10^{{{e}}}\\)" if latex else f"{mantissa}×10<sup>{e}</sup>"

def band_label(band, latex=False):
    "Where the band starts, as the tables label it"
    return power(band[0], latex)

def latex_stack(lines):
    "The lines one above another in a single table cell"
    if len(lines) == 1:
        return lines[0]
    return "\\begin{tabular}[t]{@{}l@{}}" + " \\\\ ".join(lines) + "\\end{tabular}"

def latex_duration(x):
    "duration(x), with a thin space before the unit"
    d = duration(x)
    number = d.rstrip("nµms")
    return f"{number}\\,{d[len(number):]}"
