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


#pragma once

#include <stddef.h>
#include <stdint.h>

#include "counters.h"

using Function = void(uint32_t n, uint32_t k, uint32_t* result);

struct Point {
    Function* function;
    uint32_t n, k;
    uint64_t seed;
};

struct Budget {
    // Time each stack position for at least this long, in seconds, and over
    // at least this many batches of calls
    double min_time;
    uint64_t min_batches;
    // How many stack positions to time at, 16 bytes apart; a point whose
    // call takes min_time or longer is timed at the first only
    unsigned positions;
    // A point whose first call takes this long, in seconds, is timed by that
    // call alone
    double once_above;
};

// Time a point, and print a line for each stack position it was timed at:
// the position's offset in bytes, the calls in a batch, the number of
// batches, the time per call in the fastest batch and in the median one, the
// minor page faults over all the batches, and what each counter counted per
// call in the fastest batch.
void time_point(const Point& point, const Budget& budget, const Counters& counters);

// Make this many calls one after another at one stack position, and print a
// line for each: its time, and what each counter counted.
void time_calls(const Point& point, uint64_t calls, size_t pad, const Counters& counters);
