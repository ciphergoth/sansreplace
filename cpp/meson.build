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
    default_options : ['buildtype=debugoptimized', 'c_std=c11',
'cpp_std=c++17'])

add_project_arguments(
    '-O3',
    language : ['c', 'cpp'])
libsrc = [
    'donothing.cpp',
    'randbelow.c',
    'timing.cpp',
    'quadraticreject.cpp',
    'rejectionsample.cpp',
    'ranksb.cpp',
    'floydf2.cpp',
    'quadraticf2.cpp',
    'iterativechoose.cpp',
    'reservoirsample.cpp',
    'vitterd.cpp',
    'hiddenshuffle.cpp',
    'select.cpp',
    'hsel.cpp',
    'cardchoose.cpp',
]
lib = shared_library('sansreplace',
                        libsrc,
                        install : false)
