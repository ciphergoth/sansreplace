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


// macOS only declares these when asked for the 2008 edition of the standard
// they were dropped from
#define _XOPEN_SOURCE 700

#include "stack.h"

#include <stdlib.h>
#include <ucontext.h>

// macOS deprecates them too, but has nothing else that puts a stack exactly
// where it's told to
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static const size_t STACK_SIZE = 8 << 20;
// The largest page we run with is the 16 KB of an Apple Silicon Mac
static const size_t PAGE = 16384;

static const std::function<void()>* running;

static void run() {
    (*running)();
}

void on_stack(size_t pad, const std::function<void()>& f) {
    static char* base = static_cast<char*>(aligned_alloc(PAGE, STACK_SIZE));
    running = &f;
    ucontext_t here, there;
    getcontext(&there);
    there.uc_stack.ss_sp = base;
    there.uc_stack.ss_size = STACK_SIZE - pad;
    there.uc_link = &here;
    makecontext(&there, run, 0);
    swapcontext(&here, &there);
}
