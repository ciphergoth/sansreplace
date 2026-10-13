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


# Hidden Shuffle, from Shekelyan and Cormode, "Sequential Random Sampling
# Revisited: Hidden Shuffle Method" (AISTATS 2021), with the corrections from
# the errata in the authors' repository; the code printed in the paper is
# biased.
#
# Picture a Fisher-Yates shuffle of [0, n) that stops after k steps: step i
# swaps position i with a uniformly chosen position in [i, n), and the sample
# is whatever ends up in the first k positions. Call [0, k) the low positions
# and [k, n) the high ones. Step i reaches a high position with probability
# (n-k)/(n-i), independently of the other steps, and when it does, which high
# position it reaches is uniform and independent of everything else.
#
# 1. Count h, the number of steps that reach a high position.
# 2. The first time a high position is reached, its value joins the sample.
#    If it is reached again, it holds a low value swapped there earlier, so the
#    sample gains a low value instead. So draw h high positions with
#    replacement: the distinct ones are the sample's high values, and each
#    repeat adds one to the number of low values.
# 3. Given how many values are low, they are a uniformly random subset of
#    [0, k) of that size, by symmetry. Choose them sequentially.
#
# Both draws produce positions largest first, and we report each position x
# as n-1-x, so the output comes out in ascending order.
#
# The C++ and Rust versions follow the same steps.

import math
import random

def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    d = []
    # Step 1: step i stays low with probability p_i = (k-i)/(n-i), and every
    # p_i is at most q = k/n. So imagine a uniform u_i for each step: skip
    # geometrically to the next step with u_i < q, then accept it as staying
    # low with probability p_i / q.
    h = 0
    if 0 < k < n:
        h = k
        q = k / n
        i = 0
        while True:
            i += int(math.log(1 - random.random()) / math.log1p(-q))
            if i >= k:
                break
            if random.random() * q < (k - i) / (n - i):
                h -= 1
            i += 1
    # Step 2: draw h high positions k + s, 0 <= s < n-k, with replacement,
    # largest first: the largest of h uniform reals in [0, 1) is distributed
    # as U^(1/h) (Bentley and Saxe). Repeats come out next to each other.
    low = k - h
    a = 1.0
    prev = n - k
    while h > 0:
        a *= random.random() ** (1 / h)
        s = int(a * (n - k))
        if s < prev:
            d.append(n - 1 - (k + s))
            prev = s
        else:
            low += 1
        h -= 1
    # Step 3: choose `low` of the k low positions, largest first, using
    # Vitter's Algorithm A. Choosing `low` of the m positions that remain, the
    # number s of positions skipped before the next chosen one has
    # P(S > s) = C(m-s-1, low) / C(m, low); f tracks P(S <= s).
    m = k
    while low > 0:
        u = random.random()
        s = 0
        f = low / m
        while f < u and s < m - low:
            f = 1 - (1 - low / (m - s - 1)) * (1 - f)
            s += 1
        m -= s + 1
        low -= 1
        d.append(n - 1 - m)
    return d

def random_choose(n, k):
    d = sorted_choose(n, k)
    for i in range(k):
        j = random.randrange(i+1)
        d[i], d[j] = d[j], d[i]
    return d
