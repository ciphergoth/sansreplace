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

import subprocess

def get(gitdir):
    rp = subprocess.run(["git", "rev-parse", "HEAD"], cwd=gitdir, check=True,
        stdout=subprocess.PIPE, errors='strict')
    gitstate = rp.stdout.strip()
    di = subprocess.run(["git", "diff-index", "--quiet", "HEAD", "--"],
        cwd=gitdir)
    if di.returncode != 0:
        gitstate += "+"
    return gitstate
