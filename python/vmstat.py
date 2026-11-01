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


# The state of memory. A benchmark that needs more memory than is free pays for
# it in time: macOS compresses pages or swaps them out, and Linux reclaims
# memory as it allocates, evicting pages it may have to read back in.

import pathlib
import subprocess
import sys

# The counters that show memory pressure while something runs, from vm_stat on
# macOS and /proc/vmstat on Linux
if sys.platform == "darwin":
    COUNTERS = ["Compressions", "Decompressions", "Swapins", "Swapouts"]
else:
    COUNTERS = ["pswpin", "pswpout", "pgmajfault", "pgscan_direct"]

def output(*cmd):
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout

def vm_stat():
    "vm_stat's page size and its fields, as numbers"
    lines = output("vm_stat").splitlines()
    page_size = int(lines[0].split("page size of ")[1].split()[0])
    fields = {}
    for line in lines[1:]:
        name, value = line.rsplit(":", 1)
        fields[name.strip('"')] = int(value.strip().rstrip("."))
    return page_size, fields

def proc_fields(path):
    "The fields of /proc/vmstat or /proc/meminfo, as numbers"
    fields = {}
    for line in pathlib.Path(path).read_text().splitlines():
        name, value = line.split()[:2]
        fields[name.rstrip(":")] = int(value)
    return fields

def counters():
    "The current values of COUNTERS"
    if sys.platform == "darwin":
        _, fields = vm_stat()
    else:
        fields = proc_fields("/proc/vmstat")
    return [fields[c] for c in COUNTERS]

def snapshot():
    "How much memory is free, compressed and swapped, in bytes"
    return {**(snapshot_macos() if sys.platform == "darwin" else snapshot_linux()),
        "memory_counters": " ".join(COUNTERS)}

def snapshot_macos():
    page_size, fields = vm_stat()
    swap = output("sysctl", "-n", "vm.swapusage").split()
    return {
        "memory_bytes": int(output("sysctl", "-n", "hw.memsize")),
        "memory_free_bytes": page_size * fields["Pages free"],
        "memory_compressed_bytes": page_size * fields["Pages stored in compressor"],
        "memory_compressor_bytes": page_size * fields["Pages occupied by compressor"],
        "swap_used": swap[swap.index("used") + 2],
    }

# /proc/meminfo gives sizes in kB
def snapshot_linux():
    fields = proc_fields("/proc/meminfo")
    return {
        "memory_bytes": 1024 * fields["MemTotal"],
        "memory_free_bytes": 1024 * fields["MemFree"],
        "memory_available_bytes": 1024 * fields["MemAvailable"],
        "swap_used_bytes": 1024 * (fields["SwapTotal"] - fields["SwapFree"]),
    }
