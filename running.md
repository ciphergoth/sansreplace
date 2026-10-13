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
cargo run --release -- time --sorttype random --nk 1000000 1000
cargo run --release -- test 5 3 --algorithm hsel
```

To run the full C++ and Rust benchmarks in both orders (this takes several hours):

```
python/bench_all results/my-run
```

This times each algorithm on its own, writing `results/my-run/cpp-random/hsel.csv` and so on, so
after changing one algorithm you only need to time that one, with `--algorithm`, and copy its CSVs
into `results/latest`. Each CSV records the machine, OS, compilers and power state it was timed
with, and the tools that compare algorithms warn if these differ. Timings on battery or in Low
Power Mode are not comparable with timings on mains power.

## Building the paper

The paper's figures and tables are generated from the CSVs in `results/latest`. After changing
those, regenerate them with:

```
cd python
./make_plots
./all_least_sq
./cover --latex ../latex/cover-table.tex
```

`make_plots` also writes the animated graphs in `results/latest`. Then build the PDF:

```
cd latex
./do_build
```

This writes `latex/cardchoose.pdf`; the first run downloads `latexrun` into `latex/build`. The
build prints some warnings from `latexrun` and from the LaTeX packages that can be ignored.
