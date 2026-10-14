
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

# The four cases we time, each a directory of a results directory, with a
# title for tables.
CASES = [
    ("cpp-random", "C++, random order"),
    ("cpp-sorted", "C++, sorted order"),
    ("rust-random", "Rust, random order"),
    ("rust-sorted", "Rust, sorted order"),
]
