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

#include <functional>

// Run f on a stack of our own, whose top is pad bytes below a page boundary;
// pad must be a multiple of 16. Where the stack the process starts on lies
// depends on its environment, its arguments and the kernel's randomisation,
// and where the stack lies within its page and its cache line changes some
// algorithms' times by 20% or more: see measurement.md.
void on_stack(size_t pad, const std::function<void()>& f);
