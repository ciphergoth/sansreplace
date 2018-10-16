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

project('sansreplace', ['c', 'cpp'],
    default_options : ['buildtype=debugoptimized', 'c_std=c11', 'cpp_std=c++2a'])
add_project_arguments('-Wall', '-Wno-pointer-sign', '-fno-strict-aliasing',
    language : ['c', 'cpp'])

src = [
    'cardchoose.c',
    'donothing.c',
    'main.cpp',
]
cipherbench = executable('sansreplace', src)
