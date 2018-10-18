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

#include <stdint.h>

#include <algorithm>
#include <unordered_map>

#include "randbelow.h"

extern "C" void selby_fy(uint32_t n, uint32_t k, uint32_t* result) {
    std::unordered_map<uint32_t, uint32_t> options(2*k);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(n);
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
