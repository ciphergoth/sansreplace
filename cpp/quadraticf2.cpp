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
#include <unordered_set>

#include "randbelow.h"

static inline bool contains(uint32_t needle, uint32_t size, const uint32_t* haystack) {
    for (uint32_t i = 0; i < size; i++) {
        if (haystack[i] == needle) {
            return true;
        }
    }
    return false;
}

static void quadraticf2(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k; i++) {
        uint32_t m = n + i - k;
        uint32_t r = randbelow(m + 1);
        if (contains(r, i, result)) {
            result[i] = m;
        } else {
            result[i] = r;
        }
    }
}

extern "C" void sorted_quadraticf2(uint32_t n, uint32_t k, uint32_t* result) {
    quadraticf2(n, k, result);
    std::sort(result, result + k);
}

extern "C" void random_quadraticf2(uint32_t n, uint32_t k, uint32_t* result) {
    quadraticf2(n, k, result);
    for (uint32_t i = 1; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i];
        result[i] = result[r];
        result[r] = t;
    }
}
