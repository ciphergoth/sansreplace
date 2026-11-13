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

#include <string>
#include <vector>

// A group of the processor's performance counters, counting this process's
// work in user mode. They are counted together, so the group must be no
// bigger than the number of counters the processor has. With no events, it
// counts nothing.
class Counters {
  public:
    static constexpr size_t MAX = 6;
    // As the kernel returns a group: how many counters, then their values
    struct Counts {
        uint64_t size;
        uint64_t value[MAX];
    };

    // Each event is NAME=TYPE:CONFIG, with the type and config that
    // perf_event_open takes: type 0 for the kernel's generic hardware events
    // and 4 for the processor's own, and the config in hex
    explicit Counters(const std::vector<std::string>& events);
    size_t size() const { return size_; }
    void read(Counts& counts) const;

  private:
    size_t size_ = 0;
    [[maybe_unused]] int leader_ = -1;
};
