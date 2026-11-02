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

# The Rust implementations, from the library rust/src/lib.rs builds, timed as
# load_cpp times the C++ ones

import json
import subprocess

import cffi

from load_cpp import list_functions


def setup_module(rustdir):
    # Cargo reads .cargo/config.toml, with the target CPU, from the directory
    # it runs in and its parents
    out = subprocess.run(["cargo", "build", "--release", "--message-format=json-render-diagnostics"],
        cwd=rustdir, check=True, stdout=subprocess.PIPE, text=True).stdout
    [libfile] = [f for m in map(json.loads, out.splitlines())
        if m["reason"] == "compiler-artifact" and m["target"]["kind"] == ["cdylib"]
        for f in m["filenames"]]
    ffi = cffi.FFI()
    ffi.cdef("""
        typedef struct Algorithm Algorithm;
        const Algorithm* algorithm(const char* sorttype, const char* name);
        void call(const Algorithm* f, uint32_t n, uint32_t k, uint32_t* result);
        double timefunc_s(const Algorithm* f, uint32_t n, uint32_t k, uint64_t iters);
    """)
    return ffi, ffi.dlopen(libfile)

class Timeable:
    def __init__(self, sorttype, alg, ffi, lib):
        self.sorttype = sorttype
        self.alg = alg
        self._ffi = ffi
        self._lib = lib
        self._f = lib.algorithm(sorttype.encode(), alg.encode())

    def call(self, params):
        n, k = params
        buf = self._ffi.new('uint32_t[]', k)
        self._lib.call(self._f, n, k, buf)
        return list(buf)

    def time(self, params, iters):
        n, k = params
        return self._lib.timefunc_s(self._f, n, k, iters)

def get_timeables(topdir, args):
    ffi, lib = setup_module(topdir / "rust")
    res = {}
    for sorttype, alg, _ in list_functions():
        res.setdefault(sorttype, {})[alg] = Timeable(sorttype, alg, ffi, lib)
    return res
