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


#include "counters.h"

#if __linux__

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <linux/perf_event.h>
#include <sys/ioctl.h>
#include <sys/syscall.h>
#include <unistd.h>

Counters::Counters(const std::vector<std::string>& events) {
    for (auto& event : events) {
        auto eq = event.find('=');
        auto colon = event.find(':');
        perf_event_attr attr{};
        attr.size = sizeof attr;
        attr.type = std::stoul(event.substr(eq + 1, colon - eq - 1));
        attr.config = std::stoull(event.substr(colon + 1), nullptr, 16);
        attr.exclude_kernel = 1;
        attr.exclude_hv = 1;
        attr.disabled = leader_ == -1;
        attr.read_format = PERF_FORMAT_GROUP;
        int fd = syscall(SYS_perf_event_open, &attr, 0, -1, leader_, 0);
        if (fd == -1) {
            fprintf(stderr, "%s: %s\n", event.c_str(), strerror(errno));
            exit(1);
        }
        if (leader_ == -1) {
            leader_ = fd;
        }
        size_++;
    }
    if (leader_ != -1) {
        ioctl(leader_, PERF_EVENT_IOC_ENABLE, PERF_IOC_FLAG_GROUP);
    }
}

void Counters::read(Counts& counts) const {
    if (leader_ != -1) {
        ::read(leader_, &counts, (1 + size_) * sizeof(uint64_t));
    }
}

#else

// Only Linux lets an unprivileged process count, and main.cpp offers the
// option only there
Counters::Counters(const std::vector<std::string>&) {}
void Counters::read(Counts&) const {}

#endif
