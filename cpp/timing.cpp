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

#include "timing.h"

#include <algorithm>
#include <ratio>
#include <vector>

#include "testfunc.h"

static std::chrono::duration<double> timefunc(uint32_t iters, uint32_t n, uint32_t k,
    void (*func)(uint32_t n, uint32_t k, uint32_t *result)) {
    auto result = std::vector<uint32_t>(n);
    auto start = std::chrono::system_clock::now();
    for (uint32_t i = 0; i < iters; i++) {
        func(n, k, &result[0]);
    }
    auto end = std::chrono::system_clock::now();
    return end - start;
}

std::chrono::duration<double> timefunc_for(
    std::chrono::duration<double> totake,
    uint32_t n, uint32_t k,
    void (*func)(uint32_t n, uint32_t k, uint32_t *result)) {
    uint32_t iters = 1;
    for (;;) {
        auto t = timefunc(iters, n, k, func);
        if (t * 100 >= totake) {
            if (t < totake) {
                iters = static_cast<uint32_t>(iters * totake / t);
                t = timefunc(iters, n, k, func);
            }
            return t / iters;
        }
        iters *= 10;
    }
}

struct timeable totime[] = {
//    {"donothing", donothing},
    {"cardchoose", cardchoose},
    {"fisheryates", fisheryates},
    {"floydf2", floydf2},
    {"iterativechoose", iterativechoose},
    {"rejectionsample", rejectionsample},
    {"selby_fy", selby_fy},
    {"", nullptr}
};
