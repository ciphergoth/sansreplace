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


# The state of memory on macOS, from vm_stat and sysctl. A benchmark that
# needs more memory than is free makes macOS compress pages, or swap them
# out, and pays for it in time.

import subprocess

# The vm_stat counters that show memory pressure while something runs
COUNTERS = ["Compressions", "Decompressions", "Swapins", "Swapouts"]

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

def counters():
    "The current values of COUNTERS, in pages"
    _, fields = vm_stat()
    return [fields[c] for c in COUNTERS]

def snapshot():
    "How much memory is free, compressed and swapped, in bytes"
    page_size, fields = vm_stat()
    swap = output("sysctl", "-n", "vm.swapusage").split()
    return {
        "memory_bytes": int(output("sysctl", "-n", "hw.memsize")),
        "memory_free_bytes": page_size * fields["Pages free"],
        "memory_compressed_bytes": page_size * fields["Pages stored in compressor"],
        "memory_compressor_bytes": page_size * fields["Pages occupied by compressor"],
        "swap_used": swap[swap.index("used") + 2],
    }
