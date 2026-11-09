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

// Floyd's F2, as in floydf2.cpp, with a bitset of n bits as its set: each
// step tests and sets one bit, and sorted output comes from scanning the
// bitset, with no sort.

#include <stdint.h>
#include <stdlib.h>

#include <algorithm>
#include <bit>
#include <memory>

#include "randbelow.h"

using Bitset = std::unique_ptr<uint64_t[], decltype(&free)>;

// Aligned to a cache line, as SELECT's array is, and cleared explicitly, so
// that clearing it is the only pass over it before the draws
static Bitset new_bitset(size_t words) {
    size_t size = (words * sizeof(uint64_t) + 63) & ~size_t{63};
    Bitset bits(static_cast<uint64_t*>(aligned_alloc(64, size)), free);
    std::fill(bits.get(), bits.get() + words, 0);
    return bits;
}

// Set bit x, and say whether it was already set
static inline bool test_and_set(uint64_t* bits, uint32_t x) {
    uint64_t mask = uint64_t{1} << (x & 63);
    bool was = bits[x >> 6] & mask;
    bits[x >> 6] |= mask;
    return was;
}

extern "C" void sorted_bitsetf2(uint32_t n, uint32_t k, uint32_t* result) {
    size_t words = (size_t{n} + 63) / 64;
    Bitset bits = new_bitset(words);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        if (test_and_set(bits.get(), r)) {
            test_and_set(bits.get(), m);
        }
    }
    uint32_t j = 0;
    for (size_t w = 0; w < words; w++) {
        for (uint64_t b = bits[w]; b != 0; b &= b - 1) {
            result[j++] = static_cast<uint32_t>(w * 64) + std::countr_zero(b);
        }
    }
}

// As random_floydf2: on a collision, m goes in a random earlier place, and
// what was there goes at the end
extern "C" void random_bitsetf2(uint32_t n, uint32_t k, uint32_t* result) {
    Bitset bits = new_bitset((size_t{n} + 63) / 64);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        if (!test_and_set(bits.get(), r)) {
            result[i] = r;
        } else {
            test_and_set(bits.get(), m);
            uint32_t ix = randbelow(i);
            result[i] = result[ix];
            result[ix] = m;
        }
    }
}
