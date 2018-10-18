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

#include "simdxorshift128plus.h"

static avx_xorshift128plus_key_t mykey;

static __m256i buf;
static uint32_t consumed;

void rand_init() {
    // Deterministic seeding for now
    avx_xorshift128plus_init(324, 4444, &mykey);
    consumed = sizeof(__m256i) / sizeof(uint32_t);
}

static uint32_t random32() {
    if (consumed == sizeof(__m256i) / sizeof(uint32_t)) {
        buf = avx_xorshift128plus(&mykey);
        consumed = 0;
    }
    return ((uint32_t*)&buf)[consumed++];
}

// inspired by https://lemire.me/blog/2016/06/30/fast-random-shuffling/
uint32_t randbelow(uint32_t range) {
    uint64_t random32bit = random32();
    uint64_t multiresult = random32bit * range;
    uint32_t leftover = (uint32_t)multiresult;
    if (leftover < range) {
        uint32_t threshold = -range % range;
        while (leftover < threshold) {
            random32bit = random32();
            multiresult = random32bit * range;
            leftover = (uint32_t)multiresult;
        }
    }
    return multiresult >> 32;
}
