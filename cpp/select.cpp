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
#include <stdlib.h>

#include <algorithm>
#include <memory>

#include "randbelow.h"

extern "C" void random_select(uint32_t n, uint32_t k, uint32_t* result) {
    // Unlike std::vector, this leaves the array uninitialised, so filling it
    // is the only pass over it. It's aligned to a cache line, since malloc
    // only promises 16 bytes, and how many of the fill's stores straddle two
    // cache lines would otherwise depend on where malloc happened to put it.
    // aligned_alloc needs a size that's a multiple of the alignment.
    size_t size = (n * sizeof(uint32_t) + 63) & ~size_t{63};
    std::unique_ptr<uint32_t[], decltype(&free)> options(
        static_cast<uint32_t*>(aligned_alloc(64, size)), free);
    for (uint32_t i = 0; i < n; i++) {
        options[i] = i;
    }
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = i + randbelow(n - i);
        result[i] = options[r];
        options[r] = options[i];
    }
}

extern "C" void sorted_select(uint32_t n, uint32_t k, uint32_t* result) {
    random_select(n, k, result);
    std::sort(result, result + k);
}
