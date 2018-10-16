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

#include <algorithm>
#include <random>

#include "testfunc.h"

static std::mt19937_64 generator;

void cardchoose(int n, int k, int *result) {
    auto t = n - k + 1;
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(0, t + i - 1);
        int r = distribution(generator);
        if (r < t) {
            result[i] = r;
        } else {
            result[i] = result[r - t];
        }
    }
    std::sort(result, result + k);
#if 0
    for (int i = 0; i < k; i++) {
        result[i] += i;
    }
#else
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(0, i);
        int r = distribution(generator);
        auto t = result[i] + i;
        result[i] = result[r];
        result[r] = t;
    }
#endif
}
