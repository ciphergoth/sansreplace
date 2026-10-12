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

# Vitter's Algorithm D, from "An Efficient Algorithm for Sequential Random
# Sampling" (ACM TOMS 13(1), 1987), following the implementation in its
# appendix. Each step draws the number of values to skip, S, before the next
# chosen value, by rejection sampling from a continuous approximation to its
# distribution; this takes O(1) expected time, so the whole algorithm takes
# O(k) expected time. It falls back on Vitter's Algorithm A, which draws S
# directly in O(S) time, once n is at most alpha_inv * k.

import random

ALPHA_INV = 13

def skips_a(n, k):
    "Algorithm A: yield the gaps between the k values chosen from n, in order"
    top = n - k
    while k >= 2:
        v = random.random()
        s = 0
        quot = top / n
        while quot > v:
            s += 1
            top -= 1
            n -= 1
            quot *= top / n
        yield s
        n -= 1
        k -= 1
    if k == 1:
        yield int(n * random.random())

def skips_d(n, k):
    "Algorithm D: yield the gaps between the k values chosen from n, in order"
    if k == 0:
        return
    ninv = 1 / k
    vprime = random.random() ** ninv
    qu1 = n - k + 1
    threshold = ALPHA_INV * k
    while k > 1 and threshold < n:
        nmin1inv = 1 / (k - 1)
        while True:
            # Step D2: generate U and X
            while True:
                x = n * (1 - vprime)
                s = int(x)
                if s < qu1:
                    break
                vprime = random.random() ** ninv
            u = random.random()
            # Step D3: accept?
            y1 = (u * n / qu1) ** nmin1inv
            vprime = y1 * (1 - x / n) * (qu1 / (qu1 - s))
            if vprime <= 1:
                break
            # Step D4: accept?
            y2 = 1.0
            top = n - 1
            if k - 1 > s:
                bottom = n - k
                limit = n - s
            else:
                bottom = n - s - 1
                limit = qu1
            for _ in range(limit, n):
                y2 *= top / bottom
                top -= 1
                bottom -= 1
            if n / (n - x) >= y1 * y2 ** nmin1inv:
                vprime = random.random() ** nmin1inv
                break
            vprime = random.random() ** ninv
        # Step D5: select the (S + 1)st value
        yield s
        n -= s + 1
        k -= 1
        ninv = nmin1inv
        qu1 -= s
        threshold -= ALPHA_INV
    if k > 1:
        yield from skips_a(n, k)
    else:
        yield int(n * vprime)

def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    d = []
    j = 0
    for s in skips_d(n, k):
        j += s
        d.append(j)
        j += 1
    return d

def random_choose(n, k):
    d = sorted_choose(n, k)
    for i in range(k):
        j = random.randrange(i+1)
        d[i], d[j] = d[j], d[i]
    return d
