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

import pathlib
import platform
import os
import subprocess
import sys

KEYS = ["machine", "os", "cxx", "cxxlib", "rustc", "power"]

def output(*cmd):
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout

def get():
    return {**(get_macos() if sys.platform == "darwin" else get_linux()),
        # Meson builds with $CXX, which need not be the c++ on the PATH
        "cxx": output(os.environ["CXX"], "--version").splitlines()[0],
        "cxxlib": cxx_library(),
        "rustc": output("rustc", "--version").strip(),
    }

# The same compiler can use either C++ standard library, whose std::sort and
# hash tables differ; each defines its version macro in <version>
def cxx_library():
    macros = {}
    for line in output(os.environ["CXX"], "-x", "c++", "-dM", "-E", "-include", "version",
            "/dev/null").splitlines():
        _, name, *value = line.split(maxsplit=2)
        macros[name] = value[0] if value else ""
    if "_LIBCPP_VERSION" in macros:
        return f"libc++ {macros['_LIBCPP_VERSION']}"
    return f"libstdc++ {macros['_GLIBCXX_RELEASE']} {macros['__GLIBCXX__']}"

def get_macos():
    lowpower = [line.split()[1] for line in output("pmset", "-g").splitlines()
        if line.split()[:1] == ["lowpowermode"]]
    return {
        "machine": output("sysctl", "-n", "hw.model").strip(),
        "os": output("sw_vers", "-productVersion").strip()
            + " " + output("sw_vers", "-buildVersion").strip(),
        "power": output("pmset", "-g", "batt").splitlines()[0]
            + ", low power mode " + lowpower[0],
    }

# Servers have no battery or low power mode, so there is no power entry; on
# EC2 the DMI product name is the instance type
def get_linux():
    [cpu] = {line.split(":", 1)[1].strip()
        for line in pathlib.Path("/proc/cpuinfo").read_text().splitlines()
        if line.startswith("model name")}
    product = pathlib.Path("/sys/devices/virtual/dmi/id/product_name").read_text().strip()
    return {
        "machine": f"{product}, {cpu}",
        "os": platform.freedesktop_os_release()["PRETTY_NAME"] + " " + platform.release(),
    }
