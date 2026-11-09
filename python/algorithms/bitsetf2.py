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

# Floyd's F2, as in floydf2.py, with a bitset of n bits as its set: sorted
# output comes from scanning the bitset, with no sort. Here the bitset has a
# byte per bit, for clarity.

import random

def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    bits = bytearray(n)
    for m in range(n - k, n):
        r = random.randrange(m + 1)
        if bits[r]:
            bits[m] = 1
        else:
            bits[r] = 1
    return [x for x in range(n) if bits[x]]

def random_choose(n, k):
    "k distinct integers 0 <= x < n, random"
    bits = bytearray(n)
    d = []
    for i, m in enumerate(range(n - k, n)):
        r = random.randrange(m + 1)
        if bits[r]:
            # As in random-order Floyd's F2: m goes in a random earlier place,
            # and what was there goes at the end
            bits[m] = 1
            ix = random.randrange(i)
            d.append(d[ix])
            d[ix] = m
        else:
            bits[r] = 1
            d.append(r)
    return d
