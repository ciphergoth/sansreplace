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
#include <vector>

#include "randbelow.h"
#include "testfunc.h"

void fisheryates(uint32_t n, uint32_t k, uint32_t *result) {
    std::vector<uint32_t> options(n);
    for (uint32_t i = 0; i < n; i++) {
        options[i] = i;
    }
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = i + randbelow(n - i);
        result[i] = options[r];
        options[r] = options[i];
    }
}
