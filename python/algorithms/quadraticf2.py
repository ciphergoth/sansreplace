# Copyright 2018 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import random

def random_choose(n, k):
    d = []
    for i in range(k):
        m = n - k + i
        r = random.randrange(m + 1)
        for j in range(i):
            if d[j] == r:
                d[j] = m
                break
        d.append(r)
    return d

def sorted_choose(n, k):
    d = random_choose(n, k)
    d.sort()
    return d
