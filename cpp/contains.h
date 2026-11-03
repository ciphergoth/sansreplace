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

#include <stdint.h>

// Scanning every entry without returning early lets the compiler vectorise
// this loop. std::ranges::contains scans long arrays faster, but libc++ makes
// it a call to wmemchr, which makes it slower for the short arrays that the
// quadratic algorithms are for: inside quadratic rejection, at k = 34 it took
// 5.2 ns per number against this loop's 2.7.
static inline bool contains(uint32_t needle, uint32_t size, const uint32_t* haystack) {
    bool found = false;
    for (uint32_t i = 0; i < size; i++) {
        found |= haystack[i] == needle;
    }
    return found;
}
