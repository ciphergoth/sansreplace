/*
 * Copyright 2018 Google LLC
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

#include <stdint.h>

#include <algorithm>

#include "contains.h"
#include "randbelow.h"

extern "C" void random_quadraticf2(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        // The entries are distinct, so at most one matches; writing every
        // entry without a break lets the compiler vectorise this loop.
        for (uint32_t j = 0; j < i; j++) {
            result[j] = result[j] == r ? m : result[j];
        }
        result[i] = r;
    }
}

// Before sorting only the set matters, so where r is already present, add m
// instead of moving the old r to m: the set is the same, and the scan only
// reads.
extern "C" void sorted_quadraticf2(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        result[i] = contains(r, i, result) ? m : r;
    }
    std::sort(result, result + k);
}

