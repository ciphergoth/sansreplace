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

// Vitter's Algorithm D, from "An Efficient Algorithm for Sequential Random
// Sampling" (ACM TOMS 13(1), 1987), following the implementation in its
// appendix. See python/algorithms/vitterd.py for an overview.

#include <stdint.h>

#include <algorithm>
#include <cmath>

#include "randbelow.h"

static const uint32_t ALPHA_INV = 13;

// Algorithm A: write the k values chosen from the n values starting at j
static inline void vittera(uint32_t n, uint32_t k, uint32_t j, uint32_t* result) {
    uint32_t top = n - k;
    while (k >= 2) {
        double v = randdouble();
        double quot = (double)top / n;
        while (quot > v) {
            j++;
            top--;
            n--;
            quot *= (double)top / n;
        }
        *result++ = j++;
        n--;
        k--;
    }
    if (k == 1) {
        *result = j + randbelow(n);
    }
}

static inline void vitterd(uint32_t n, uint32_t k, uint32_t* result) {
    if (k == 0) {
        return;
    }
    uint32_t j = 0;
    double ninv = 1.0 / k;
    double vprime = std::pow(randdouble(), ninv);
    uint32_t qu1 = n - k + 1;
    uint64_t threshold = (uint64_t)ALPHA_INV * k;
    while (k > 1 && threshold < n) {
        double nmin1inv = 1.0 / (k - 1);
        uint32_t s;
        for (;;) {
            // Step D2: generate U and X
            double x;
            for (;;) {
                x = n * (1 - vprime);
                s = (uint32_t)x;
                if (s < qu1) {
                    break;
                }
                vprime = std::pow(randdouble(), ninv);
            }
            double u = randdouble();
            // Step D3: accept?
            double y1 = std::pow(u * n / qu1, nmin1inv);
            vprime = y1 * (1 - x / n) * ((double)qu1 / (qu1 - s));
            if (vprime <= 1) {
                break;
            }
            // Step D4: accept?
            double y2 = 1;
            double top = n - 1;
            double bottom;
            uint32_t limit;
            if (k - 1 > s) {
                bottom = n - k;
                limit = n - s;
            } else {
                bottom = n - s - 1;
                limit = qu1;
            }
            for (uint32_t t = limit; t < n; t++) {
                y2 *= top / bottom;
                top--;
                bottom--;
            }
            if (n / (n - x) >= y1 * std::pow(y2, nmin1inv)) {
                vprime = std::pow(randdouble(), nmin1inv);
                break;
            }
            vprime = std::pow(randdouble(), ninv);
        }
        // Step D5: select the (S + 1)st value
        j += s;
        *result++ = j++;
        n -= s + 1;
        k--;
        ninv = nmin1inv;
        qu1 -= s;
        threshold -= ALPHA_INV;
    }
    if (k > 1) {
        vittera(n, k, j, result);
    } else {
        // The appendix reuses vprime here, but it can round to exactly 1,
        // which would give j + n
        *result = j + randbelow(n);
    }
}

extern "C" void sorted_vitterd(uint32_t n, uint32_t k, uint32_t* result) {
    vitterd(n, k, result);
}

extern "C" void random_vitterd(uint32_t n, uint32_t k, uint32_t* result) {
    vitterd(n, k, result);
    for (uint32_t i = 0; i < k; i++) {
        uint32_t r = randbelow(i + 1);
        auto t = result[i];
        result[i] = result[r];
        result[r] = t;
    }
}
