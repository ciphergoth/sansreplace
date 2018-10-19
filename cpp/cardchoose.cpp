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

static inline void cardchoose(uint32_t n, uint32_t k, uint32_t* result) {
    auto t = n - k + 1;
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(t + i);
        if (r < t) {
            result[i] = r;
        } else {
            result[i] = result[r - t];
        }
    }
    std::sort(result, result + k);
}

extern "C" void sorted_cardchoose(uint32_t n, uint32_t k, uint32_t* result) {
    cardchoose(n, k, result);
    for (uint32_t i = 0; i < k; i++) {
        result[i] += i;
    }
}

extern "C" void random_cardchoose(uint32_t n, uint32_t k, uint32_t* result) {
    cardchoose(n, k, result);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i] + i;
        result[i] = result[r];
        result[r] = t;
    }
}
