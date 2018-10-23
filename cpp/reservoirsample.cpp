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

extern "C" void random_reservoirsample(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        result[i] = result[r];
        result[r] = i;
    }
    for (uint32_t i = k; i < n; i++) {
        uint32_t r = randbelow(i + 1);
        if (r < k) {
            result[r] = i;
        }
    }
}

extern "C" void sorted_reservoirsample(uint32_t n, uint32_t k, uint32_t* result) {
    for (uint32_t i = 0; i < k; i++) {
        result[i] = i;
    }
    for (uint32_t i = k; i < n; i++) {
        uint32_t r = randbelow(i + 1);
        if (r < k) {
            result[r] = i;
        }
    }
    std::sort(result, result + k);
}
