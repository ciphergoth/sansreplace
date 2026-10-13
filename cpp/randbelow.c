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

#include "randbelow.h"

// Xoshiro256++ (Blackman and Vigna, https://prng.di.unimi.it/): fast enough
// that the benchmarks don't measure it, and with no weak bits, so every bit
// of every output can be used.

static uint64_t state[4];

static inline uint64_t rotl(uint64_t x, int k) {
    return (x << k) | (x >> (64 - k));
}

static uint64_t random64() {
    uint64_t result = rotl(state[0] + state[3], 23) + state[0];
    uint64_t t = state[1] << 17;
    state[2] ^= state[0];
    state[3] ^= state[1];
    state[1] ^= state[2];
    state[0] ^= state[3];
    state[2] ^= t;
    state[3] = rotl(state[3], 45);
    return result;
}

// Each output supplies two 32-bit words
static uint64_t buf;
static int have_half;

static uint32_t random32() {
    if (have_half) {
        have_half = 0;
        return (uint32_t)(buf >> 32);
    }
    buf = random64();
    have_half = 1;
    return (uint32_t)buf;
}

// SplitMix64, the seeding generator recommended for Xoshiro
static uint64_t splitmix64(uint64_t* x) {
    uint64_t z = (*x += 0x9e3779b97f4a7c15);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
    z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
    return z ^ (z >> 31);
}

void rand_init() {
    // Deterministic seeding for now
    uint64_t seed = 324;
    for (int i = 0; i < 4; i++) {
        state[i] = splitmix64(&seed);
    }
    have_half = 0;
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

// Uniform in [0, 1) with 53 random bits: the top 53 bits of one output
double randdouble() {
    return (double)(random64() >> 11) * (1.0 / 9007199254740992.0);
}
