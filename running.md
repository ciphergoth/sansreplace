# Running this software

This is experimental software; it exists only to compare the various implementations, and it will
change in arbitrary ways as I extend my tests.

This is not an officially supported Google product.

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
after changing one algorithm you only need to time that one and copy its CSVs into
`results/latest`. `--run LANG/ORDER/ALGORITHM` times only what it names, where any of the three
parts may be `*` to mean all of them; it may be repeated, and `--dry-run` lists what would be
timed without timing it. For example, this times `select` in C++ in both orders, and both quadratic
algorithms in every case:

```
python/bench_all --run 'cpp/*/select' --run '*/*/quadraticreject' --run '*/*/quadraticf2' results/my-run
```

Options for `bench_all` itself go before the output directory; anything after it is passed on to the
timers. On macOS it keeps the machine from idle-sleeping while it runs, so give one run everything
you want timed, rather than running it several times in a row: the machine can fall asleep between
one run and the next. Closing the lid still sleeps it, but both
`time_choose` and the Rust `time` notice when the machine slept during a point, and time that
point again. Each CSV records the machine, OS, compilers, C++ standard library and, on macOS,
power state it was timed with, and the tools that compare algorithms warn if these differ. Timings on battery or in
Low Power Mode are not comparable with timings on mains power.

On x86-64, both the C++ and the Rust are compiled for `x86-64-v3`, the baseline with AVX2, rather
than the compilers' default of the original x86-64 baseline, whose vectors are only SSE2; see
[`cpp/meson.build`](cpp/meson.build) and [`.cargo/config.toml`](.cargo/config.toml). On Linux, as
on macOS, the C++ is built with clang and its own standard library, libc++, rather than GCC's
libstdc++, whose `std::sort` and hash tables are different code; see [`devenv.nix`](devenv.nix).

### The results CSVs

Each row of a results CSV starts with its kind:

* `config,NAME,VALUE`: an option the timer was run with.
* `info,NAME,VALUE`: the commit and setup it was timed with. `bench_all` also records the state of
  memory when the run starts, and in `memory_counters` the names of the counters in the
  `memory` rows.
* `result,ALG,N,K,SECONDS`: the time of one call.
* `memory,ALG,N,K,C1,C2,C3,C4`: how much four counters of the whole machine's memory activity grew
  while that point was timed. Large numbers mean the timing was slowed by memory pressure, from
  this benchmark or anything else running. On macOS they are the pages compressed,
  decompressed, swapped in and swapped out, from `vm_stat`. On Linux they are the pages swapped in
  and out, the major page faults and the pages scanned by direct reclaim, from `/proc/vmstat`
  (`pswpin`, `pswpout`, `pgmajfault` and `pgscan_direct`); servers often have no swap, and then
  memory pressure shows up as direct reclaim, which evicts pages that may have to be read back in
  through major faults.
* `faults,ALG,N,K,MINOR_FAULTS,CALLS`: the minor page faults the timer itself took in the pass that
  timed that point, and how many calls that pass made. A minor fault maps a page in without reading
  anything from disk, as when memory that the allocator gave back to the kernel is used again; an
  algorithm that allocates memory and frees it on every call may pay for this on every call,
  depending on the allocator. Results from before these rows were added don't have them.
* `skipped,ALG,N,K`: a point not timed because the algorithm took more than `--time-limit` seconds
  at a smaller or equal _k_ for the same _n_, or did so at the same _k_ for two different smaller
  _n_. Some algorithms are slow only near _k_ = _n_, so going over the limit once only skips
  larger _k_ for that _n_. Results from before these rows were added don't have them.

Recording the setup and memory works on macOS and Linux. On Linux, the machine recorded is the
DMI product name, which on EC2 is the instance type, and the CPU model.

## Regenerating the Markdown

The graphs and pairwise comparisons that the Markdown pages show are generated from the CSVs in
`results/latest`, and checked in under it. After changing those CSVs, regenerate them with:

```
python/make_plots
python/pareto
python/pairwise
```

`make_plots` writes the animated graphs and `pareto` the scattergrams for
[choosing algorithms](choosing.md), both into `results/latest/graphs`, and `pairwise` writes
[`pairwise-ratio.md`](results/latest/pairwise-ratio.md) and
[`pairwise-difference.md`](results/latest/pairwise-difference.md). The tables in
[choosing algorithms](choosing.md) come from `python/cover --measure ratio` and
`--measure difference`, and the ones in the [results](results.md) from `python/summarise`, as
those pages say.

`python/regions CASE A B` compares two algorithms by region of _n_/_k_ and _k_, as for the claims in
the [results](results.md); `B` can be `others`, for the fastest of all the other algorithms.

## The web explorer

[The explorer](https://ciphergoth.github.io/sansreplace/) shows each algorithm's time/_k_ at a
point (_n_, _k_) chosen on a triangle. It is the page in `web/`, with the results from
`results/latest` that `python/export_web` writes as one JSON file into `web/generated`, which is
not checked in. To try it locally:

```
python/export_web --out web/generated/data.json
python3 -m http.server -d web
```

On every push to `master` that changes `web/`, `python/` or `results/latest`, the `pages` workflow
does the same and publishes the result to GitHub Pages.

## Building the paper

```
cd latex
./do_build
```

This generates the paper's figures and tables from the CSVs in `results/latest` into
`latex/generated`, which is not checked in, then writes `latex/cardchoose.pdf`. The first run
downloads `latexrun` into `latex/build`. The build prints some warnings from `latexrun` and from
the LaTeX packages that can be ignored.
