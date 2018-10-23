#!/usr/bin/env python3

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

import gc
import importlib
import time

class Timeable:
    def __init__(self, choose):
        self._choose = choose

    def call(self, params):
        return self._choose(*params)

    def time(self, params, iters):
        gc.disable()
        start = time.process_time()
        for _ in range(iters):
            self._choose(*params)
        finish = time.process_time()
        gc.enable()
        return finish - start

def get_timeables(topdir):
    res = {}
    for m in ['cardchoose', 'donothing', 'fisheryates', 'floydf2',
        'iterativechoose', 'quadraticreject', 'rejectionsample',
        'reservoirsample', 'selby_fy']:
        md = importlib.import_module(f"algorithms.{m}")
        for prefix in ["donothing", "random", "sorted"]:
            name = f"{prefix}_choose"
            f = getattr(md, name, None)
            if f is not None:
                res.setdefault(prefix, {})[m] = Timeable(f)
    return res
