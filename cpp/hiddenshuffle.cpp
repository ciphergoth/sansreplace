/*
 * Copyright 2026 Paul Crowley
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

// Hidden Shuffle, from Shekelyan and Cormode, "Sequential Random Sampling
// Revisited: Hidden Shuffle Method" (AISTATS 2021), with the corrections from
// the errata in the authors' repository. See python/algorithms/hiddenshuffle.py
// for an overview.

#include <stdint.h>

#include <cmath>

#include "randbelow.h"

static inline void hiddenshuffle(uint32_t n, uint32_t k, uint32_t* result) {
    // Step 1: h is the number of the first k Fisher-Yates steps that reach
    // [k, n)
    uint32_t h = 0;
    if (0 < k && k < n) {
        h = k;
        double q = (double)k / n;
        double logq = std::log1p(-q);
        uint32_t i = 0;
        for (;;) {
            double g = std::log(1 - randdouble()) / logq;
            if (g >= k - i) {
                break;
            }
            i += (uint32_t)g;
            if (randdouble() * q < (double)(k - i) / (n - i)) {
                h--;
            }
            i++;
            if (i >= k) {
                break;
            }
        }
    }
    // Step 2: h draws with replacement from [k, n), largest first
    uint32_t low = k - h;
    uint32_t range = n - k;
    uint32_t prev = range;
    double a = 1;
    while (h > 0) {
        a *= std::pow(randdouble(), 1.0 / h);
        uint32_t s = (uint32_t)(a * range);
        if (s < prev) {
            *result++ = n - 1 - (k + s);
            prev = s;
        } else {
            low++;
        }
        h--;
    }
    // Step 3: choose low of the k positions in [0, k), largest first, by
    // Vitter's Algorithm A
    uint32_t m = k;
    while (low > 0) {
        double u = randdouble();
        uint32_t s = 0;
        double f = (double)low / m;
        while (f < u && s < m - low) {
            f = 1 - (1 - (double)low / (m - s - 1)) * (1 - f);
            s++;
        }
        m -= s + 1;
        low--;
        *result++ = n - 1 - m;
    }
}

extern "C" void sorted_hiddenshuffle(uint32_t n, uint32_t k, uint32_t* result) {
    hiddenshuffle(n, k, result);
}

extern "C" void random_hiddenshuffle(uint32_t n, uint32_t k, uint32_t* result) {
    hiddenshuffle(n, k, result);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i];
        result[i] = result[r];
        result[r] = t;
    }
}
