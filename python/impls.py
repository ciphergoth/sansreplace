# Copyright 2018 Google LLC
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

"""The implementations in each language, as programs with one command line:
cpp/main.cpp, rust/src/main.rs and, for the Python ones, python/sansreplace.
A program runs once for each point timed, so that no point's time depends on
what the process did before."""

import json
import subprocess
import sys

SEED = 324

def point_seed(n, k):
    """The seed every batch of calls at (n, k) starts from, so that a point's
    batches make the same draws in any run, whatever was timed before.
    Distinct for every (n, k)."""
    return SEED ^ (n << 32 | k)

def cpp_command(topdir, build_dir):
    cppdir = topdir / "cpp"
    if build_dir is None:
        build_dir = cppdir / "build/host"
    # meson setup runs in cppdir, so a relative path must be made absolute
    build_dir = build_dir.resolve()
    if not (build_dir / "build.ninja").is_file():
        subprocess.run(["meson", "setup", build_dir], cwd=cppdir, check=True)
    subprocess.run(["ninja", "-C", build_dir], check=True)
    targets = json.loads(subprocess.run(["meson", "introspect", build_dir, "--targets"],
        check=True, capture_output=True).stdout)
    [exe] = [t["filename"][0] for t in targets if t["name"] == "sansreplace"]
    return [exe]

def rust_command(topdir):
    # Cargo reads .cargo/config.toml, with the target CPU, from the directory
    # it runs in and its parents
    out = subprocess.run(["cargo", "build", "--release", "--message-format=json-render-diagnostics"],
        cwd=topdir / "rust", check=True, stdout=subprocess.PIPE, text=True).stdout
    [exe] = [m["executable"] for m in map(json.loads, out.splitlines())
        if m["reason"] == "compiler-artifact" and m["target"]["kind"] == ["bin"]]
    return [exe]

def command(topdir, language, build_dir=None):
    "Build the implementations in this language, and return the command that runs them"
    if language == "cpp":
        return cpp_command(topdir, build_dir)
    if language == "rust":
        return rust_command(topdir)
    return [sys.executable, str(topdir / "python/sansreplace")]

def point_args(order, algorithm, n, k, seed):
    return ["--order", order, "--algorithm", algorithm, "--n", str(n), "--k", str(k),
        "--seed", str(seed)]

def sample(cmd, order, algorithm, n, k, count):
    "count samples, each a list of k numbers"
    out = subprocess.run(cmd + ["sample", *point_args(order, algorithm, n, k, SEED),
        "--count", str(count)], check=True, stdout=subprocess.PIPE, text=True).stdout
    return [[int(x) for x in line.split()] for line in out.splitlines()]
