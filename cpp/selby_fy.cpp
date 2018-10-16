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
#include <unordered_map>

#include "testfunc.h"

static std::mt19937_64 generator;

void selby_fy(int n, int k, int *result) {
    std::unordered_map<int, int> options;
    std::uniform_int_distribution<int> distribution(0, n - 1);
    for (int i = 0; i < k; i++) {
        int r = distribution(generator);
        auto fr = options.find(r);
        if (fr == options.end()) {
            result[i] = r;
        } else {
            result[i] = fr->second;
        }
        auto fi = options.find(i);
        if (fi == options.end()) {
            options[r] = i;
        } else {
            options[r] = fi->second;
        }
    }
}
