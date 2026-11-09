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

# b_ndebug: Meson otherwise adds bounds checks to the standard library, defining
# _LIBCPP_HARDENING_MODE for libc++ and _GLIBCXX_ASSERTIONS for libstdc++
project('sansreplace', ['c', 'cpp'],
    default_options : ['buildtype=debugoptimized', 'c_std=c23',
'cpp_std=c++23', 'b_ndebug=true'])

add_project_arguments(
    '-O3',
    language : ['c', 'cpp'])
# The x86-64 default is the 2003 baseline, with only SSE2 vectors; target the
# AVX2 baseline instead, as /.cargo/config.toml does for Rust
if host_machine.cpu_family() == 'x86_64'
    add_project_arguments(
        '-march=x86-64-v3',
        language : ['c', 'cpp'])
endif
libsrc = [
    'donothing.cpp',
    'randbelow.c',
    'timing.cpp',
    'quadraticreject.cpp',
    'quadraticf2.cpp',
    'iterativechoose.cpp',
    'reservoirsample.cpp',
    'select.cpp',
    'rejectionsample.cpp',
    'floydf2.cpp',
    'hsel.cpp',
    'ranksb.cpp',
    'vitterd.cpp',
    'hiddenshuffle.cpp',
    'cardchoose.cpp',
]
lib = shared_library('sansreplace',
                        libsrc,
                        install : false)
