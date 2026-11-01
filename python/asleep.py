# Copyright 2026 Paul Crowley
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

import sys
import time

# Of each pair, the first clock counts while the machine is asleep and the
# second doesn't, so the difference grows only while asleep
if sys.platform == "darwin":
    CLOCKS = (time.CLOCK_MONOTONIC_RAW, time.CLOCK_UPTIME_RAW)
else:
    CLOCKS = (time.CLOCK_BOOTTIME, time.CLOCK_MONOTONIC)

# How long the machine has spent asleep since it booted. A timing during which
# this grows can't be trusted.
def total():
    return time.clock_gettime(CLOCKS[0]) - time.clock_gettime(CLOCKS[1])

# Reading the two clocks one after the other makes the difference jitter by
# far less than this, and the shortest sleep is far longer
THRESHOLD_S = 0.01

# After waking, timings run slow for several seconds, so wait this long
# before timing again; the first timing after the wait also runs slow, so
# discard it
SETTLE_S = 5

def settle():
    time.sleep(SETTLE_S)
