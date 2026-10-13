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

# RANKSB, from Nijenhuis and Wilf, "Combinatorial Algorithms for Computers
# and Calculators" (2nd edition, 1978), chapter 4.
#
# Divide [0, n) into k bins of nearly equal size. First decide how many of
# the chosen values fall in each bin, then choose that many values within
# each bin. The C++ and Rust versions follow the book's program, which packs
# everything into the k words of the output; this version keeps the counts in
# a separate array, which is easier to follow.

import random

def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    # Bin l is [lo(l), lo(l+1))
    def lo(l):
        return l * n // k
    # Rejection sampling, drawing until k distinct values are accepted,
    # chooses a uniformly random k-subset. Simulate it keeping only how many
    # values each bin has accepted: a draw in bin l duplicates an accepted
    # value with probability counts[l] / (bin size), so reject it when it
    # lands among the first counts[l] values of its bin, without needing to
    # know which values those were.
    counts = [0] * k
    accepted = 0
    while accepted < k:
        x = random.randrange(n)
        l = ((x + 1) * k - 1) // n
        if x - lo(l) >= counts[l]:
            counts[l] += 1
            accepted += 1
    # Given the counts, the values in each bin are a uniformly random subset
    # of that size, independently of the other bins. Bins hold about one value
    # each, so a quadratic method is fine.
    d = []
    for l in range(k):
        s = []
        while len(s) < counts[l]:
            x = random.randrange(lo(l), lo(l + 1))
            if x not in s:
                s.append(x)
        s.sort()
        d.extend(s)
    return d

def random_choose(n, k):
    d = sorted_choose(n, k)
    for i in range(k):
        j = random.randrange(i+1)
        d[i], d[j] = d[j], d[i]
    return d
