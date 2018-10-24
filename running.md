# Running this software

This is experimental software; it exists only to compare the various implementations, and it will
change in arbitrary ways as I extend my tests.

The following build instructions are written for users of Ubuntu and other
Debian-derived Linux systems; adjust as needed for your own platform.

* Install [Python 3](https://www.python.org/): `apt-get install python3`
    * Even the C++ tests depend on Python 3.
* Install [Ninja](https://ninja-build.org/): `apt-get install ninja-build`
    * Only needed for the C++ tests.
* Optionally, create a Python 3 [virtual environment](https://docs.python.org/3/library/venv.html)
  so you don't have to install globally:
    * `python3 -m venv venv`
    * `. ./venv/bin/activate`
* Install needed Python packages:
    * `pip install attrs matplotlib meson`
* Try it out

```
cd python
./test_choose --help
./time_choose --help
./plot --help
./scatter --help
```

The C++ tools automatically create the build directory and build the code before running if needed;
try eg `./test_choose 5 3`.
