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

#include <boost/sort/pdqsort/pdqsort.hpp>
#include <boost/unordered/unordered_flat_set.hpp>

#include "randbelow.h"

extern "C" void random_rejectionsample(uint32_t n, uint32_t k, uint32_t* result) {
    boost::unordered_flat_set<uint32_t> done;
    done.reserve(k);
    for (uint32_t i = 0; i < k;) {
        uint32_t r = randbelow(n);
        auto iresult = done.insert(r); // Try to insert
        if (iresult.second) { // Did insert succeed, or was it already present?
            result[i++] = r;
        }
    }
}

extern "C" void sorted_rejectionsample(uint32_t n, uint32_t k, uint32_t* result) {
    random_rejectionsample(n, k, result);
    boost::sort::pdqsort(result, result + k);
}
