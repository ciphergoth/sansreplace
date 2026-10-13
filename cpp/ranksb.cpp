/*
 * Copyright 2026 Paul Crowley
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// RANKSB, from Nijenhuis and Wilf, "Combinatorial Algorithms for Computers
// and Calculators" (2nd edition, 1978), chapter 4, following the book's
// program, which uses only the k words of the output. See
// python/algorithms/ranksb.py for an overview. The book indexes from 1 and
// uses 0 to mark an empty slot, so we do the same internally, through A(i),
// and subtract 1 from each value at the end.

#include <stdint.h>

#include "randbelow.h"

static void ranksb(uint32_t n, uint32_t k, uint32_t* result) {
    if (k == 0) {
        return;
    }
    auto A = [result](uint32_t i) -> uint32_t& { return result[i - 1]; };
    // Bin l is the values zero(l) < x <= zero(l + 1)
    auto zero = [n, k](uint64_t l) { return (uint32_t)((l - 1) * n / k); };
    auto bin = [n, k](uint64_t x) { return (uint32_t)(1 + (x * k - 1) / n); };
    // (A) A(l) starts at the "zero point" of bin l
    for (uint32_t l = 1; l <= k; l++) {
        A(l) = zero(l);
    }
    // (B) Choose how many values fall in each bin: A(l) - zero(l) counts them
    for (uint32_t c = k; c > 0;) {
        uint32_t x = 1 + randbelow(n);
        uint32_t l = bin(x);
        if (x > A(l)) {
            A(l)++;
            c--;
        }
    }
    // (C) Move the entries for nonempty bins to the left
    uint32_t p = 0;
    for (uint32_t i = 1; i <= k; i++) {
        uint32_t m = A(i);
        A(i) = 0;
        if (m != zero(i)) {
            p++;
            A(p) = m;
        }
    }
    // (D) Reserve space for each bin's values, from the right, marking the
    // rightmost slot of each with the bin number
    uint32_t s = k;
    while (p > 0) {
        uint32_t l = bin(A(p));
        uint32_t ds = A(p) - zero(l);
        A(p) = 0;
        A(s) = l;
        s -= ds;
        p--;
    }
    // (E)-(H) Fill each bin's slots, from the right, with distinct values
    // from that bin in increasing order
    uint32_t l = k;
    uint32_t r = 0, m0 = 0, m = 0;
    while (l > 0) {
        if (A(l) != 0) {
            // (E) A new bin: its slots end at r, and m values are left in it
            r = l;
            m0 = 1 + zero(A(l));
            m = zero((uint64_t)A(l) + 1) - m0 + 1;
        }
        // (F) Choose one of the m values not yet chosen
        uint32_t x = m0 + randbelow(m);
        // (G) Skip over the values already chosen, shifting them left
        uint32_t i = l + 1;
        while (i <= r && x >= A(i)) {
            x++;
            A(i - 1) = A(i);
            i++;
        }
        // (H) Insert x
        A(i - 1) = x;
        m--;
        l--;
    }
    for (uint32_t i = 0; i < k; i++) {
        result[i]--;
    }
}

extern "C" void sorted_ranksb(uint32_t n, uint32_t k, uint32_t* result) {
    ranksb(n, k, result);
}

extern "C" void random_ranksb(uint32_t n, uint32_t k, uint32_t* result) {
    ranksb(n, k, result);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i];
        result[i] = result[r];
        result[r] = t;
    }
}
