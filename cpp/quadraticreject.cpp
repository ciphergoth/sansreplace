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

#include "randbelow.h"

static inline bool contains(uint32_t needle, uint32_t size, const uint32_t* haystack) {
    for (uint32_t i = 0; i < size; i++) {
        if (haystack[i] == needle) {
            return true;
        }
    }
    return false;
}

extern "C" void random_quadraticreject(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k;) {
        uint32_t r = randbelow(n);
        if (!contains(r, i, result)) {
            result[i++] = r;
        }
    }
}

extern "C" void sorted_quadraticreject(uint32_t n, uint32_t k, uint32_t* result) {
    random_quadraticreject(n, k, result);
    std::sort(result, result + k);
}
