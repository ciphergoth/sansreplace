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

import importlib
import subprocess
import sys

import cffi

samplers = [
    "quadraticreject",
    "cardchoose",
    "fisheryates",
    "floydf2",
    "iterativechoose",
    "rejectionsample",
    "selby_fy"
]

def setup_module(cppdir, functions):
    b = cppdir / "build/host"
    if not b.is_dir():
        subprocess.run(["meson",  b], check=True)
    subprocess.run(["ninja", "-C", b], check=True)
    ffi = cffi.FFI()
    ffi.cdef("""
        void rand_init();
        double timefunc_s(void (*func)(uint32_t n, uint32_t k, uint32_t* result),
                          uint32_t n, uint32_t k, uint32_t iters);
        void callchoose(void (*func)(uint32_t n, uint32_t k, uint32_t* result),
                        uint32_t n, uint32_t k, uint32_t* result);
    """)
    source = """
        #include "randbelow.h"
        #include "timing.h"

        void callchoose(void (*func)(uint32_t n, uint32_t k, uint32_t* result),
                        uint32_t n, uint32_t k, uint32_t* result) {
            func(n, k, result);
        }
    """
    for f in functions:
        ffi.cdef(f"void (*const {f})(uint32_t n, uint32_t k, uint32_t* result);")
        source += f"void {f}(uint32_t n, uint32_t k, uint32_t* result);\n"
    ffi.set_source("sansreplace", source,
        include_dirs=[str(cppdir)],
        library_dirs=[str(b)],
        runtime_library_dirs=[str(b)],
        libraries=["sansreplace"],
        extra_compile_args=['-std=c11'],
        extra_link_args=[f"-Wl,-rpath,{b}"])
    ffi.compile(tmpdir=str(b))
    sys.path.append(str(b))
    sansreplace = importlib.import_module("sansreplace")
    sansreplace.lib.rand_init()
    return sansreplace

def list_functions():
    yield ("donothing", "donothing", "donothing")
    for prefix in ["sorted", "random"]:
        for t in samplers:
            yield (prefix, t, f"{prefix}_{t}")

class Timeable:
    def __init__(self, module, funcname):
        self._module = module
        self._funcname = funcname
        self._fpointer = getattr(module.lib, funcname)

    def call(self, params):
        n, k = params
        buf = self._module.ffi.new('uint32_t[]', k)
        self._module.lib.callchoose(self._fpointer, n, k, buf)
        return list(buf)

    def time(self, params, iters):
        n, k = params
        return self._module.lib.timefunc_s(self._fpointer, n, k, iters)

def get_timeables(topdir):
    module = setup_module(topdir / "cpp", [f[2] for f in list_functions()])
    res = {}
    for prefix, t, funcname in list_functions():
        res.setdefault(prefix, {})[t] = Timeable(module, funcname)
    return res