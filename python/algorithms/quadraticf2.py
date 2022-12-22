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

def set_choose(n, k):
    "k distinct integers 0 <= x < n"
    d = set()
    for i in range(n - k, n):
        j = random.randrange(i+1)
        if j in d:
            d.add(i)
        else:
            d.add(j)
    return d

def random_choose(n, k):
    d = set_choose(n, k)
    rd = [None] * k
    for i, v in enumerate(d):
        j = random.randrange(i+1)
        rd[i] = rd[j]
        rd[j] = v
    return rd

def sorted_choose(n, k):
    d = list(set_choose(n, k))
    d.sort()
    return d
