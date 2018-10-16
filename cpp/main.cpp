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

#include <algorithm>
#include <chrono>
#include <iostream>
#include <ratio>
#include <string>
#include <vector>

#include "randbelow.h"
#include "testfunc.h"
#include "timing.h"

static void time_all(int n, int k) {
    struct timeable *tm = &totime[0];
    std::chrono::seconds sec(1);
    double base_time_s = 0;

    for (tm = totime; tm->func; tm++) {
        auto time_s = timefunc_for(sec, n, k, tm->func).count();
        std::cout << tm->name << std::endl;
        std::cout << "    " << time_s << " s" << std::endl;
        std::cout << "    " << time_s/k << " s/k" << std::endl;
        std::cout << "    " << time_s/n << " s/n" << std::endl;
        if (tm == totime) {
            base_time_s = time_s;
        } else {
            std::cout << "    " << time_s / base_time_s << " vs " << totime[0].name << std::endl;
        }
    }
}

int main() {
    rand_init();
    for (int n = 10; n < 10000000; n *= 10) {
        for (int k = 8; k < n; k *= 10) {
            std::cout << "n, k = " << n << ", " << k << std::endl;
            time_all(n, k);
            std::cout << std::endl;
        }
    }
    return 0;
}
