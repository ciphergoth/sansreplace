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
#include <vector>

#include "randbelow.h"

extern "C" void sorted_iterativechoose(uint32_t n, uint32_t k, uint32_t* result) {
    uint32_t ix = 0;
    for (uint32_t i = 0; i < n; i++) {
        uint32_t r = randbelow(n - i);
        if (r < k - ix) {
            result[ix++] = i;
        }
    }
}

extern "C" void random_iterativechoose(uint32_t n, uint32_t k, uint32_t* result) {
    sorted_iterativechoose(n, k, result);
    for (uint32_t i = 1; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i];
        result[i] = result[r];
        result[r] = t;
    }
}
