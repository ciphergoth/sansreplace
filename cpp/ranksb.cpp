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
// python/algorithms/ranksb.py for an overview; this uses the same bins, the
// same test for accepting a draw, and the same steps, but keeps each bin's
// count in the output array.
//
// We depart from the book's program in two ways. It indexes slots, values and
// bins from 1, so that 0 can mark an empty slot; we index from 0. And after
// compacting the counts, its step (D) marks out each bin's slots, writing the
// bin number into the rightmost, and then steps (E)-(H) scan for those marks
// to fill the bins. We fill each bin's slots as soon as we know where they
// are, which needs no marks.

#include <stdint.h>

#include "randbelow.h"

static void ranksb(uint32_t n, uint32_t k, uint32_t* result) {
    // Bin l is the values lo(l) <= x < lo(l + 1)
    auto lo = [n, k](uint64_t l) { return (uint32_t)(l * n / k); };
    auto bin = [n, k](uint64_t x) { return (uint32_t)(((x + 1) * k - 1) / n); };
    // (A) result[l] holds lo(l) + the number of values chosen in bin l. Each
    // nonempty bin's entry is then one more than a value in the bin, so it
    // says which bin it counts.
    for (uint32_t l = 0; l < k; l++) {
        result[l] = lo(l);
    }
    // (B) Simulate rejection sampling: a draw duplicates a value already
    // chosen in its bin with probability count / (bin size), so reject it if
    // it lands among the first count values of its bin
    for (uint32_t c = k; c > 0;) {
        uint32_t x = randbelow(n);
        uint32_t l = bin(x);
        if (x >= result[l]) {
            result[l]++;
            c--;
        }
    }
    // (C) Move the entries for nonempty bins to the left
    uint32_t p = 0;
    for (uint32_t l = 0; l < k; l++) {
        if (result[l] != lo(l)) {
            result[p] = result[l];
            p++;
        }
    }
    // An entry says which bin it counts and how many values that bin needs
    struct Bin {
        uint32_t l, lo, count;
    };
    auto decode = [&](uint32_t e) {
        uint32_t l = bin(e - 1);
        return Bin{l, lo(l), e - lo(l)};
    };
    // Fill each bin's slots, working from the right. The q nonempty bins
    // before the one result[q] counts each need at least one slot, so its
    // slots are all at q or later, and filling them overwrites no entry we
    // have yet to read. Decoding takes divisions, so we decode each entry
    // before filling the bin to its right, while the CPU has other work to
    // do, rather than leaving the fill to wait for them.
    uint32_t s = k;
    Bin b = p > 0 ? decode(result[p - 1]) : Bin{};
    for (uint32_t q = p; q-- > 0;) {
        Bin next = q > 0 ? decode(result[q - 1]) : Bin{};
        // m values in the bin are not yet chosen
        uint32_t m = lo((uint64_t)b.l + 1) - b.lo;
        // Fill the slots s - count to s - 1, from the right, with distinct
        // values from the bin in increasing order
        for (uint32_t r = s; r-- > s - b.count;) {
            // (F) Choose one of the m values not yet chosen
            uint32_t x = b.lo + randbelow(m);
            // (G) Skip over the values already chosen, shifting them left
            uint32_t i = r + 1;
            while (i < s && x >= result[i]) {
                x++;
                result[i - 1] = result[i];
                i++;
            }
            // (H) Insert x
            result[i - 1] = x;
            m--;
        }
        s -= b.count;
        b = next;
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
