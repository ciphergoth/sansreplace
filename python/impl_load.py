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
import pathlib

def add_arguments(p):
    p.add_argument("--language", default="cpp", choices=["cpp", "python"])
    p.add_argument("--build-dir", type=pathlib.Path)

def get_timeables(args, topdir):
    return importlib.import_module(f"load_{args.language}").get_timeables(topdir, args)
