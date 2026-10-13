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
# Sampling" (ACM TOMS 13(1), 1987).
#
# Like iterative choosing, this walks through [0, n) in order, but instead of
# deciding about each value in turn, it draws S, the number of values to skip
# before the next chosen one. When choosing k of the n values that remain,
#
#     P(S = s) = C(n-s-1, k-1) / C(n, k)    for 0 <= s <= n-k
#
# since the remaining k-1 chosen values must come from the n-s-1 after it.
#
# This version states the method directly. The C++ and Rust versions follow
# the paper's appendix, which computes the same thing with fewer random
# numbers, mostly by reusing one uniform variate between steps.

import random

# Use Algorithm D while n > ALPHA_INV * k, Algorithm A otherwise
ALPHA_INV = 13

def skip_probability(n, k, s):
    "P(S = s) when choosing k of n, as a product of min(s, k-1) factors"
    # C(n-s-1, k-1) / C(n, k) = (k/n) C(n-s-1, k-1) / C(n-1, k-1), and the
    # last ratio expands as either of two products of falling factors.
    p = k / n
    if s < k - 1:
        for i in range(s):
            p *= (n - k - i) / (n - 1 - i)
    else:
        for i in range(k - 1):
            p *= (n - s - 1 - i) / (n - 1 - i)
    return p

def skip_d(n, k):
    "Algorithm D: draw S by rejection from a continuous approximation, k >= 2"
    # Picture the distribution of S as a histogram: over [s, s+1), a bar of
    # height P(S = s). The curve c g(x), where
    #
    #     g(x) = (k/n) (1 - x/n)^(k-1)    and    c = n / (n-k+1),
    #
    # lies above every bar. Pick a point uniformly under the curve: x is
    # n (1 - V^(1/k)), the smallest of k uniform reals in [0, n), whose
    # density is g, and the height is uniform in [0, c g(x)). If the point is
    # under the bar in its column, s = floor(x), return s; otherwise try
    # again. Accepted points are uniform over the histogram, so s has exactly
    # the right distribution, and since the curve has c times the area of the
    # histogram, this takes about 1 + k/n tries on average.
    #
    # Inside each bar is a lower bar of height h(s) = (k/n) (1 - s/q)^(k-1).
    # A point under it is accepted without computing P(S = s) exactly, so the
    # O(min(s, k)) exact test runs only for points in the thin sliver between
    # the two.
    q = n - k + 1
    while True:
        x = n * (1 - random.random() ** (1 / k))
        s = int(x)
        if s >= q:
            continue
        envelope = (n / q) * (k / n) * (1 - x / n) ** (k - 1)
        threshold = random.random() * envelope
        if threshold <= (k / n) * (1 - s / q) ** (k - 1):
            return s
        if threshold <= skip_probability(n, k, s):
            return s

def skip_a(n, k):
    "Algorithm A: draw S by inversion, in O(S) time"
    # P(S > s) is the probability that the first s+1 values are all skipped
    v = random.random()
    s = 0
    tail = (n - k) / n
    while tail > v:
        s += 1
        tail *= (n - k - s) / (n - s)
    return s

def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    d = []
    j = 0
    while k > 0:
        if k == 1:
            s = random.randrange(n)
        elif n > ALPHA_INV * k:
            s = skip_d(n, k)
        else:
            s = skip_a(n, k)
        j += s
        d.append(j)
        j += 1
        n -= s + 1
        k -= 1
    return d

def random_choose(n, k):
    d = sorted_choose(n, k)
    for i in range(k):
        j = random.randrange(i+1)
        d[i], d[j] = d[j], d[i]
    return d
