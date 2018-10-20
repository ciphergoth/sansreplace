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

#include "timing.h"

#include <algorithm>
#include <chrono>
#include <ratio>
#include <vector>

double timefunc_s(void (*func)(uint32_t n, uint32_t k, uint32_t* result), uint32_t n, uint32_t k,
                  uint32_t iters) {
    auto result = std::vector<uint32_t>(k);
    auto start = std::chrono::system_clock::now();
    for (uint32_t i = 0; i < iters; i++) {
        func(n, k, &result[0]);
    }
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<double> duration = end - start;
    return duration.count();
}
