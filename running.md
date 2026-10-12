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

The Rust implementation is separate. `sweep` times every algorithm over a range of _n_, _k_ values
and writes the results to a CSV file; `time` times every algorithm for a single _n_, _k_; and
`test`, like `test_choose`, checks one algorithm for fairness. They use the random order variants
unless you pass `--sorttype sorted`:

```
cd rust
cargo run --release -- sweep results.csv
cargo run --release -- sweep --sorttype sorted results.csv
cargo run --release -- time 1000000 1000
cargo run --release -- test 5 3 --algorithm hsel
```
