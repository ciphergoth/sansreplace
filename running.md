# Running this software

This is experimental software; it exists only to compare the various implementations, and it will
change in arbitrary ways as I extend my tests.

The development environment is managed with [devenv](https://devenv.sh/), which provides Python
and its needed packages, Meson, Ninja, and Rust.

* [Install devenv](https://devenv.sh/getting-started/).
* Either:
    * install [direnv](https://direnv.net/) and run `direnv allow` in the top directory of this
      repository, so the environment is loaded whenever you `cd` into it, or
    * run `devenv shell` in the top directory to start a shell with the environment loaded.
* Try it out. While all of the scripts provide help from the command line, you will almost certainly
need to read the source code to figure out what it does.

```
cd python
./test_choose --help
./time_choose --help
./plot --help
./scatter --help
```

Even the C++ tests are driven from Python. The C++ tools automatically create the build directory
and build the code before running if needed; try eg `./test_choose 5 3`.

The Rust implementation is separate, but its subcommands mirror the Python tools and take the same
options: `time` is like `time_choose`, and writes CSV files in the same format, and `test` is like
`test_choose`:

```
cd rust
cargo run --release -- time --values small --values fib --csv results.csv
cargo run --release -- time --sorttype sorted --nk 1000000 1000
cargo run --release -- test 5 3 --algorithm hsel
```

To run the full C++ and Rust benchmarks in both orders, writing a CSV for each to a directory
(this takes several hours):

```
python/bench_all results/my-run
```
