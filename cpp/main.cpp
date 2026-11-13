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


// The C++ implementations, as a program that python/bench runs once for each
// point it times, and python/test_choose runs for samples:
//
//   sansreplace list
//   sansreplace sample --order ORDER --algorithm NAME --n N --k K --seed SEED --count COUNT
//   sansreplace time --order ORDER --algorithm NAME --n N --k K --seed SEED ...
//   sansreplace calls --order ORDER --algorithm NAME --n N --k K --seed SEED --calls CALLS ...
//
// ORDER is random, sorted, component for the parts of the algorithms in
// components.cpp, or, for the one that does nothing, donothing. See timing.h
// for what time and calls print.

#include <stdint.h>
#include <stdio.h>

#include <string>
#include <vector>

#include <CLI/CLI.hpp>

#include "counters.h"
#include "randbelow.h"
#include "timing.h"

// Same order as python/algorithms/__init__.py
#define ALGORITHMS(X)  \
    X(quadraticreject) \
    X(quadraticf2)     \
    X(iterativechoose) \
    X(reservoirsample) \
    X(select)          \
    X(bitvectorf2)     \
    X(rejectionsample) \
    X(floydf2)         \
    X(hsel)            \
    X(ranksb)          \
    X(vitterd)         \
    X(hiddenshuffle)   \
    X(cardchoose)
#define COMPONENTS(X) \
    X(count)          \
    X(fill)           \
    X(fillsort)       \
    X(fillshuffle)    \
    X(hashinsert)

extern "C" {
Function donothing;
#define X(name) Function random_##name, sorted_##name;
ALGORITHMS(X)
#undef X
#define X(name) Function component_##name;
COMPONENTS(X)
#undef X
}

struct Entry {
    std::string order, name;
    Function* function;
};

static const std::vector<Entry> ENTRIES = {
    {"donothing", "donothing", donothing},
#define X(name) {"random", #name, random_##name},
    ALGORITHMS(X)
#undef X
#define X(name) {"sorted", #name, sorted_##name},
    ALGORITHMS(X)
#undef X
#define X(name) {"component", #name, component_##name},
    COMPONENTS(X)
#undef X
};

int main(int argc, char** argv) {
    CLI::App app{"Sample without replacement, or time doing it"};
    app.require_subcommand(1);
    std::string order, name;
    Point point;
    auto add_point = [&](CLI::App* sub) {
        sub->add_option("--order", order)->required();
        sub->add_option("--algorithm", name)->required();
        sub->add_option("--n", point.n)->required();
        sub->add_option("--k", point.k)->required();
        sub->add_option("--seed", point.seed)->required();
    };
    std::vector<std::string> events;
    auto add_counters = [&](CLI::App* sub) {
#if __linux__
        sub->add_option("--counter", events,
                        "NAME=TYPE:CONFIG, a performance counter as perf_event_open takes it, "
                        "with the config in hex; may be repeated");
#endif
    };

    auto* list = app.add_subcommand("list", "Print the order and name of everything here");

    auto* sample = app.add_subcommand("sample", "Print samples, one to a line");
    add_point(sample);
    uint64_t count;
    sample->add_option("--count", count)->required();

    auto* time = app.add_subcommand("time", "Time a point, at each of several stack positions");
    add_point(time);
    add_counters(time);
    Budget budget;
    time->add_option("--min-time", budget.min_time)->required();
    time->add_option("--min-batches", budget.min_batches)->required();
    time->add_option("--positions", budget.positions)->required();
    time->add_option("--once-above", budget.once_above)->required();

    auto* calls = app.add_subcommand("calls", "Time each of a run of calls");
    add_point(calls);
    add_counters(calls);
    uint64_t ncalls;
    size_t pad;
    calls->add_option("--calls", ncalls)->required();
    calls->add_option("--pad", pad)->required();

    CLI11_PARSE(app, argc, argv);

    if (*list) {
        for (auto& e : ENTRIES) {
            printf("%s %s\n", e.order.c_str(), e.name.c_str());
        }
        return 0;
    }
    point.function = nullptr;
    for (auto& e : ENTRIES) {
        if (e.order == order && e.name == name) {
            point.function = e.function;
        }
    }
    if (point.function == nullptr) {
        fprintf(stderr, "no %s %s\n", order.c_str(), name.c_str());
        return 1;
    }
    if (point.k > point.n) {
        fprintf(stderr, "k must be at most n\n");
        return 1;
    }
    if (*sample) {
        std::vector<uint32_t> result(point.k);
        rand_init(point.seed);
        for (uint64_t i = 0; i < count; i++) {
            point.function(point.n, point.k, result.data());
            for (uint32_t j = 0; j < point.k; j++) {
                printf(j == 0 ? "%u" : " %u", result[j]);
            }
            printf("\n");
        }
    } else if (*time) {
        time_point(point, budget, Counters(events));
    } else {
        time_calls(point, ncalls, pad, Counters(events));
    }
}
