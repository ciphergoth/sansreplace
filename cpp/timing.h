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

#include <chrono>
#include <string>

extern std::chrono::duration<double> timefunc_for(
    std::chrono::duration<double> totake,
    uint32_t n, uint32_t k,
    void (*func)(uint32_t n, uint32_t k, uint32_t *result));

struct timeable {
    std::string name;
    void (*func)(uint32_t n, uint32_t k, uint32_t *result);
};

extern struct timeable totime[];
