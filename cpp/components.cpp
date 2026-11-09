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

// Parts of the algorithms, timed on their own to compare the two standard
// libraries. fill draws k random numbers below n into the output; each of the
// others does the same and then one thing more, so its time less fill's is
// what that thing costs on k random numbers. Each does that thing as the
// algorithms do.

#include <stdint.h>

#include <algorithm>
#include <unordered_set>

#include "randbelow.h"

static inline void fill(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k; i++) {
        result[i] = randbelow(n);
    }
}

extern "C" void component_fill(uint32_t n, uint32_t k, uint32_t* result) {
    fill(n, k, result);
}

extern "C" void component_fillsort(uint32_t n, uint32_t k, uint32_t* result) {
    fill(n, k, result);
    std::sort(result, result + k);
}

// The shuffle that puts the sorted algorithms' output in random order
extern "C" void component_fillshuffle(uint32_t n, uint32_t k, uint32_t* result) {
    fill(n, k, result);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i];
        result[i] = result[r];
        result[r] = t;
    }
}

// The set set-based rejection sampling keeps. Its size goes into the output,
// so that building it can't be optimised away.
extern "C" void component_hashinsert(uint32_t n, uint32_t k, uint32_t* result) {
    fill(n, k, result);
    std::unordered_set<uint32_t> done(2 * k);
    for (uint32_t i = 0; i < k; i++) {
        done.insert(result[i]);
    }
    result[0] = static_cast<uint32_t>(done.size());
}
