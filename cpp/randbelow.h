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

#pragma once

#include <stdbool.h>
#include <stdint.h>

// The generator is defined here so that it can be inlined into each
// algorithm, as Rust's is; a call for every random number would handicap the
// algorithms that draw the most.

#if __cplusplus
extern "C" {
#endif

// Xoshiro256++ (Blackman and Vigna, https://prng.di.unimi.it/): fast enough
// that the benchmarks don't measure it, and with no weak bits, so every bit
// of every output can be used.

extern uint64_t rand_state[4];

static inline uint64_t rand_rotl(uint64_t x, int k) {
    return (x << k) | (x >> (64 - k));
}

static inline uint64_t random64(void) {
    uint64_t result = rand_rotl(rand_state[0] + rand_state[3], 23) + rand_state[0];
    uint64_t t = rand_state[1] << 17;
    rand_state[2] ^= rand_state[0];
    rand_state[3] ^= rand_state[1];
    rand_state[1] ^= rand_state[2];
    rand_state[0] ^= rand_state[3];
    rand_state[2] ^= t;
    rand_state[3] = rand_rotl(rand_state[3], 45);
    return result;
}

// Each output supplies two 32-bit words
extern uint64_t rand_buf;
extern bool rand_have_half;

static inline uint32_t random32(void) {
    if (rand_have_half) {
        rand_have_half = false;
        return (uint32_t)(rand_buf >> 32);
    }
    rand_buf = random64();
    rand_have_half = true;
    return (uint32_t)rand_buf;
}

extern void rand_init(void);

// inspired by https://lemire.me/blog/2016/06/30/fast-random-shuffling/
static inline uint32_t randbelow(uint32_t range) {
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

// Uniform in [0, 1) with 53 random bits: the top 53 bits of one output
static inline double randdouble(void) {
    return (double)(random64() >> 11) * (1.0 / 9007199254740992.0);
}

#if __cplusplus
}
#endif
