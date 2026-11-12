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

// Floyd's F2, as in floydf2.cpp, with a bit vector of n bits as its set: each
// step tests and sets one bit, and sorted output comes from scanning the
// bit vector, with no sort.

#include <stdint.h>

#include <boost/dynamic_bitset.hpp>

#include "randbelow.h"

// test_set sets a bit, and says whether it was already set
using BitVector = boost::dynamic_bitset<>;

extern "C" void sorted_bitvectorf2(uint32_t n, uint32_t k, uint32_t* result) {
    BitVector bits(n);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        if (bits.test_set(r)) {
            bits.set(m);
        }
    }
    uint32_t j = 0;
    for (auto x = bits.find_first(); x != BitVector::npos; x = bits.find_next(x)) {
        result[j++] = static_cast<uint32_t>(x);
    }
}

// As random_floydf2: on a collision, m goes in a random earlier place, and
// what was there goes at the end
extern "C" void random_bitvectorf2(uint32_t n, uint32_t k, uint32_t* result) {
    BitVector bits(n);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        if (!bits.test_set(r)) {
            result[i] = r;
        } else {
            bits.set(m);
            uint32_t ix = randbelow(i);
            result[i] = result[ix];
            result[ix] = m;
        }
    }
}
