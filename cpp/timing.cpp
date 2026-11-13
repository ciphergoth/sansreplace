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


#include "timing.h"

#include <stdio.h>
#include <stdlib.h>

#include <sys/resource.h>

#include <algorithm>
#include <chrono>
#include <memory>
#include <vector>

#include "randbelow.h"
#include "stack.h"

using Result = std::unique_ptr<uint32_t[], decltype(&free)>;

static Result make_result(uint32_t k) {
    // The output is aligned to a cache line, since malloc only promises 16
    // bytes, and the quadratic algorithms scan it on every draw: how many of
    // their loads and stores straddle two cache lines would otherwise depend on
    // where malloc happened to put it. aligned_alloc needs a size that's a
    // multiple of the alignment. It's zeroed once, outside the timing, as a
    // std::vector would be: no algorithm depends on what's there, but some
    // copy an entry before writing it, and this also faults in its pages.
    size_t size = (k * sizeof(uint32_t) + 63) & ~size_t{63};
    Result result(static_cast<uint32_t*>(aligned_alloc(64, size)), free);
    std::fill(result.get(), result.get() + k, 0);
    return result;
}

static long minor_faults() {
    rusage usage;
    getrusage(RUSAGE_SELF, &usage);
    return usage.ru_minflt;
}

// The time iters calls take, in seconds, and what the counters counted
static double batch(const Point& point, uint32_t* result, uint64_t iters, const Counters& counters,
                    Counters::Counts& counts) {
    Counters::Counts before, after;
    counters.read(before);
    auto start = std::chrono::steady_clock::now();
    for (uint64_t i = 0; i < iters; i++) {
        point.function(point.n, point.k, result);
    }
    auto end = std::chrono::steady_clock::now();
    counters.read(after);
    for (size_t i = 0; i < counters.size(); i++) {
        counts.value[i] = after.value[i] - before.value[i];
    }
    return std::chrono::duration<double>(end - start).count();
}

// Every batch starts from the same seed, so that every batch of a point makes
// the same draws, in any run
static double seeded_batch(const Point& point, uint32_t* result, uint64_t iters,
                           const Counters& counters, Counters::Counts& counts) {
    rand_init(point.seed);
    return batch(point, result, iters, counters, counts);
}

static void print_position(size_t pad, uint64_t iters, const std::vector<double>& times,
                           double fastest, long faults, const Counters& counters,
                           const Counters::Counts& counts) {
    std::vector<double> sorted(times);
    std::sort(sorted.begin(), sorted.end());
    size_t mid = sorted.size() / 2;
    double median = sorted.size() % 2 == 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
    printf("%zu %llu %zu %.17g %.17g %ld", pad, static_cast<unsigned long long>(iters),
           times.size(), fastest / iters, median / iters, faults);
    for (size_t i = 0; i < counters.size(); i++) {
        printf(" %.17g", static_cast<double>(counts.value[i]) / iters);
    }
    printf("\n");
}

// Time at one stack position, and return the time per call. iters is the
// calls in a batch: zero at the first position, which sets it, so that every
// position times the same calls.
static double time_position(const Point& point, const Budget& budget, const Counters& counters,
                            uint32_t* result, size_t pad, uint64_t& iters) {
    std::vector<double> times;
    times.reserve(64);
    Counters::Counts counts, best;
    long faults = minor_faults();
    if (iters == 0) {
        double t = seeded_batch(point, result, 1, counters, counts);
        if (t >= budget.once_above) {
            times.push_back(t);
            print_position(pad, 1, times, t, minor_faults() - faults, counters, counts);
            return t;
        }
        // That call, and these, also warm up
        iters = 1;
        while (t < budget.min_time / 10) {
            iters *= 2;
            t = seeded_batch(point, result, iters, counters, counts);
        }
    } else {
        // Warm up
        seeded_batch(point, result, iters, counters, counts);
    }
    faults = minor_faults();
    double total = 0;
    double fastest = 0;
    while (times.size() < budget.min_batches || total < budget.min_time) {
        double t = seeded_batch(point, result, iters, counters, counts);
        total += t;
        if (times.empty() || t < fastest) {
            fastest = t;
            best = counts;
        }
        times.push_back(t);
    }
    print_position(pad, iters, times, fastest, minor_faults() - faults, counters, best);
    return fastest / iters;
}

void time_point(const Point& point, const Budget& budget, const Counters& counters) {
    Result result = make_result(point.k);
    uint64_t iters = 0;
    for (unsigned i = 0; i < budget.positions; i++) {
        double t;
        on_stack(16 * i, [&] {
            t = time_position(point, budget, counters, result.get(), 16 * i, iters);
        });
        if (t >= budget.min_time) {
            break;
        }
    }
}

void time_calls(const Point& point, uint64_t calls, size_t pad, const Counters& counters) {
    Result result = make_result(point.k);
    std::vector<double> times(calls);
    std::vector<Counters::Counts> counts(calls);
    on_stack(pad, [&] {
        rand_init(point.seed);
        for (uint64_t c = 0; c < calls; c++) {
            times[c] = batch(point, result.get(), 1, counters, counts[c]);
        }
    });
    for (uint64_t c = 0; c < calls; c++) {
        printf("%.17g", times[c]);
        for (size_t i = 0; i < counters.size(); i++) {
            printf(" %llu", static_cast<unsigned long long>(counts[c].value[i]));
        }
        printf("\n");
    }
}
