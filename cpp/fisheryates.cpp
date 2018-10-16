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
#include <vector>

#include "testfunc.h"

static std::mt19937_64 generator;

void fisheryates(int n, int k, int *result) {
    std::vector<int> options(n);
    for (int i = 0; i < n; i++) {
        options[i] = i;
    }
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(i, n - 1);
        int r = distribution(generator);
        result[i] = options[r];
        options[r] = options[i];
    }
}
