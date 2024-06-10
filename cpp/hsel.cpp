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

std::unordered_map<uint32_t, uint32_t> hsel_options;

extern "C" void random_hsel(uint32_t n, uint32_t k, uint32_t* result) {
    hsel_options.clear();
    for (uint32_t i = 0; i != k; ++i) {
      result[i] = i;
    }
    for (uint32_t i = 0; i != k; ++i) {
        const uint32_t r = randbelow(n--) + i;

        if (r < k) {
            std::swap(result[i], result[r]);
        } else {
            const auto res_i = result[i];
            const auto [fr, success] = hsel_options.emplace(r, res_i);
            if (success) {
                result[i] = r;
            } else {
                result[i] = fr->second;
                fr->second = res_i;
            }
        }
    }
}

extern "C" void sorted_hsel(uint32_t n, uint32_t k, uint32_t* result) {
    random_hsel(n, k, result);
    std::sort(result, result + k);
}
