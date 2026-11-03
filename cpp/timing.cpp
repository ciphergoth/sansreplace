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

#include <stdio.h>
#include <stdlib.h>

#include "timing.h"

#include <algorithm>
#include <chrono>
#include <memory>
#include <ratio>

double timefunc_s(void (*func)(uint32_t n, uint32_t k, uint32_t* result), uint32_t n, uint32_t k,
                  uint64_t iters) {
    // The output is aligned to a cache line, since malloc only promises 16
    // bytes, and the quadratic algorithms scan it on every draw: how many of
    // their loads and stores straddle two cache lines would otherwise depend on
    // where malloc happened to put it. aligned_alloc needs a size that's a
    // multiple of the alignment. It's zeroed once, outside the timing, as a
    // std::vector would be: no algorithm depends on what's there, but some
    // copy an entry before writing it, and this also faults in its pages.
    size_t size = (k * sizeof(uint32_t) + 63) & ~size_t{63};
    std::unique_ptr<uint32_t[], decltype(&free)> result(
        static_cast<uint32_t*>(aligned_alloc(64, size)), free);
    std::fill(result.get(), result.get() + k, 0);
    auto start = std::chrono::steady_clock::now();
    for (uint64_t i = 0; i < iters; i++) {
        func(n, k, result.get());
    }
    auto end = std::chrono::steady_clock::now();
    std::chrono::duration<double> duration = end - start;
    return duration.count();
}
