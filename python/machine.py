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

# The setup a benchmark runs on. Each algorithm is timed in its own run, so
# results from different runs are only comparable if these all match.

import subprocess

KEYS = ["machine", "os", "cxx", "rustc", "power"]

def output(*cmd):
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout

def get():
    lowpower = [line.split()[1] for line in output("pmset", "-g").splitlines()
        if line.split()[:1] == ["lowpowermode"]]
    return {
        "machine": output("sysctl", "-n", "hw.model").strip(),
        "os": output("sw_vers", "-productVersion").strip()
            + " " + output("sw_vers", "-buildVersion").strip(),
        "cxx": output("c++", "--version").splitlines()[0],
        "rustc": output("rustc", "--version").strip(),
        "power": output("pmset", "-g", "batt").splitlines()[0]
            + ", low power mode " + lowpower[0],
    }
