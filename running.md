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
python/bench --help
python/test_choose --help
python/test_all --help
python/plot --help
python/scatter --help
```

Both the C++ and the Rust implementations are driven from Python: each is built as a program,
which the tools build if needed and run, and the two take the same command line; see
[`cpp/main.cpp`](cpp/main.cpp) and [`rust/src/main.rs`](rust/src/main.rs).
[`python/sansreplace`](python/sansreplace) does the same for the Python implementations.
`python/bench` times them, and `python/test_choose` tests one for fairness, with `--language`
choosing which, `cpp` by default; try eg `python/test_choose 5 3`. `python/test_all` tests every
algorithm in every language at a few small points, and lists any that fail.

`python/bench --run LANG/ORDER/ALGORITHM` times what it names, where any of the three parts may be
`*` to mean all of them, and `LANG` is `cpp` or `rust`, or `python` for the Python
implementations, which `*` leaves out; it may be repeated, and `--dry-run` lists what would be timed
without timing it. Each point is timed in a process of its own, so that its time doesn't depend on
what was timed before it. With `--nk N K`, it times that point only, and compares the algorithms
at each point at the end:

```
python/bench --run 'rust/random/*' --nk 1000000 1000
```

By default it times every algorithm in C++ and Rust in both orders, over the points the published
results use; with `--out`, it writes a CSV for each, as `OUT/cpp-random/hsel.csv` and so on. This
takes several hours:

```
python/bench --out results/my-run
```

So after changing one algorithm you only need to time that one and copy its CSVs into
`results/latest`. For example, this times `select` in C++ in both orders, and both quadratic
algorithms in every case:

```
python/bench --run 'cpp/*/select' --run '*/*/quadraticreject' --run '*/*/quadraticf2' --out results/my-run
```

Some points' times vary between runs by much more than the timer's noise (see
[measurement](measurement.md)), so the published results take each point's fastest time over
several runs of the same code, made hours apart. `python/fastest` combines runs that way, and
reports each point whose slowest run took more than `--report` times its fastest, 1.2 by default:

```
python/fastest --out results/combined results/run1 results/run2 results/run3
```

`python/agree` says how well runs of the same code agree: how many points' slowest run took more
than 1.03, 1.05 and so on times their fastest, overall and by algorithm, and the points that agree
worst, with the counters each run recorded there. `python/stability` runs `bench` several times on
a few points, with 4 KB pages and 2 MB ones, and reports how far apart the runs are; it is where
the figures in [measurement](measurement.md#how-far-to-trust-a-time) come from, and needs Linux.

```
python/agree results/run1 results/run2 --row rust-random/hsel 1346269
python/stability --out results/stability --cpu 3
```

Each process warms up, then times batches of calls, each batch starting from the same seed, until
it has timed at least `--min-batches` batches over at least `--min-time` seconds, and takes the
fastest batch. It does that with its stack at each of `--positions` positions, 16 bytes apart, and
`bench` takes the fastest position, because where the stack lies changes some algorithms' times; see
[measurement](measurement.md). A point whose call takes `--min-time` or longer is timed at one
position, and one whose first call takes `--once-above` seconds is timed by that call alone.

On Linux, `bench` runs each process with address space randomisation off, and with an empty
environment everywhere. `--cpu N` pins every process to one CPU, `--huge-pages` has the C library
take memory in 2 MB pages, and `--counters` records performance counters with each point; the
counters are those of the processor the published results are timed on, listed in
[`python/bench`](python/bench).

The timer's settings, such as `--min-time` and `--time-limit`, are options too, each defaulting to
what the published results use; see `--help`. On macOS it keeps the machine from idle-sleeping while
it runs, so give one run everything you want timed, rather than running it several times in a row:
the machine can fall asleep between one run and the next. Closing the lid still sleeps it, but
`bench` notices when the machine slept during a point, and times that point again. Each CSV records
the machine, OS, compilers, C++ standard library, Boost version and, on macOS, power state it was
timed with, and the tools that compare algorithms warn if these differ. Timings on battery or in
Low Power Mode are not comparable with timings on mains power.

On x86-64, both the C++ and the Rust are compiled for `x86-64-v3`, the baseline with AVX2, rather
than the compilers' default of the original x86-64 baseline, whose vectors are only SSE2; see
[`cpp/meson.build`](cpp/meson.build) and [`.cargo/config.toml`](.cargo/config.toml). The Rust is
compiled as a single codegen unit, with link-time optimisation, so that how each algorithm is
compiled doesn't depend on how the compiler happens to split the rest of the library into parts;
see [`rust/Cargo.toml`](rust/Cargo.toml). In both languages, every function starts on a 64-byte
cache line: otherwise where a function starts depends on all the code the linker put before it, and
that alone can move a tight loop's time by 15% or more, so that changing one algorithm could change
another's timings. The C++ is built with clang, using the system's standard library: GCC's
libstdc++ on Linux and libc++ on macOS. Its sort and hash tables come from
[Boost](https://www.boost.org/) rather than the standard library: `boost::sort::pdqsort`, and
`boost::unordered_flat_set` and `unordered_flat_map`; see [`devenv.nix`](devenv.nix).

### The results CSVs

Each row of a results CSV starts with its kind:

* `config,NAME,VALUE`: an option the timer was run with.
* `info,NAME,VALUE`: the commit and setup it was timed with, the state of memory when timing
  started, and in `memory_counters` the names of the counters in the `memory` rows. Results from
  before `bench` replaced `bench_all` have these rows after the others rather than before them.
* `result,ALG,N,K,SECONDS`: the time of one call.
* `memory,ALG,N,K,C1,C2,C3,C4`: how much four counters of the whole machine's memory activity grew
  while that point was timed. Large numbers mean the timing was slowed by memory pressure, from
  this benchmark or anything else running. On macOS they are the pages compressed,
  decompressed, swapped in and swapped out, from `vm_stat`. On Linux they are the pages swapped in
  and out, the major page faults and the pages scanned by direct reclaim, from `/proc/vmstat`
  (`pswpin`, `pswpout`, `pgmajfault` and `pgscan_direct`); servers often have no swap, and then
  memory pressure shows up as direct reclaim, which evicts pages that may have to be read back in
  through major faults.
* `position,ALG,N,K,OFFSET,CALLS,BATCHES,FASTEST,MEDIAN,MINOR_FAULTS,COUNTERS...`: the timing at
  one stack position, `OFFSET` bytes below a page boundary: the calls in a batch, the number of
  batches timed, the time of one call in the fastest batch and in the median one, the minor page
  faults over all the batches, and with `--counters`, what each counter that `info,counters` names
  counted in one call of the fastest batch. The point's `result` is the smallest `FASTEST`. A
  minor fault maps a page in without reading anything from disk, as when memory that the allocator
  gave back to the kernel is used again; an algorithm that allocates memory and frees it on every
  call may pay for this on every call, depending on the allocator.
* `faults,ALG,N,K,MINOR_FAULTS,CALLS`: in results from before each point was timed in a process of
  its own, the minor page faults the timer took in the pass that timed that point, and how many
  calls that pass made.
* `skipped,ALG,N,K`: a point not timed because the algorithm took more than `--time-limit` seconds
  at a smaller or equal _k_ for the same _n_, or did so at the same _k_ for two different smaller
  _n_. Some algorithms are slow only near _k_ = _n_, so going over the limit once only skips
  larger _k_ for that _n_. Results from before these rows were added don't have them.
* `run,ALG,N,K,INDEX`: in results that `python/fastest` combined, which of the runs that
  `info,fastest_of` lists, counting from 0, the point's other rows come from. The other `info` rows
  are the first run's.

Recording the setup and memory works on macOS and Linux. On Linux, the machine recorded is the
DMI product name, which on EC2 is the instance type, and the CPU model.

## Regenerating the Markdown

The graphs, pairwise comparisons and scores by band of _k_ that the Markdown pages show are generated
from the CSVs in `results/latest`, and checked in under it. After changing those CSVs, regenerate
them with:

```
python/make_plots
python/pairwise
python/summarise --every-algorithm results/latest/by-band.md
```

`make_plots` writes the animated graphs into `results/latest/graphs`, `pairwise` writes
[`pairwise-ratio.md`](results/latest/pairwise-ratio.md) and
[`pairwise-difference.md`](results/latest/pairwise-difference.md), and `summarise` writes
[every algorithm's score in every band](results/latest/by-band.md). The tables in the
[results](results.md) come from `python/summarise` and the one in
[choosing algorithms](choosing.md) from `python/cover`, which print them as Markdown; both score
algorithms as [`python/bands.py`](python/bands.py) defines, and `python/cover --top N` lists more of
the best sets.

`python/regions CASE A B...` compares two algorithms by region of _n_/_k_ and _k_, as for the claims
in the [results](results.md); given several algorithms after `A`, it compares `A` with the fastest of
them, and given `others`, with the fastest of all the other algorithms.

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
`latex/generated`, which is not checked in, then builds it with `latexmk` and XeLaTeX, leaving the PDF at
`latex/build/latex.out/cardchoose.pdf`.

On every push to `master` that changes the paper, the results or the build, the Pages workflow
builds the paper the same way and publishes it alongside the explorer, at
<https://ciphergoth.github.io/sansreplace/cardchoose.pdf>.
