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
    d = [None] * k
    for i in range(k):
        r = random.randrange(i + 1)
        d[i] = d[r]
        d[r] = i
    for i in range(k, n):
        r = random.randrange(i + 1)
        if r < k:
            d[r] = i
    return d

def sorted_choose(n, k):
    d = list(range(k))
    for i in range(k, n):
        r = random.randrange(i + 1)
        if r < k:
            d[r] = i
    d.sort()
    return d
