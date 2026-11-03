# Making the timings trustworthy

Several times, a result that looked odd, such as one algorithm faster than another in one language
but slower in the other, or a point that changed between runs, turned out to come from how it was
measured rather than from the algorithm. This page lists what we found and what we changed, so that
the [results](results.md) measure the algorithms and the languages' standard libraries, and not
accidents of memory layout or code generation. The times quoted here are from the AWS r8a.xlarge
that the published results were timed on.

## The machine

The results are timed on an AWS r8a.xlarge, whose four vCPUs are each a whole core of an AMD EPYC
9R45, so nothing else shares the core a benchmark runs on. It is stopped between runs, and may come
back on a different host. Timing the same points on four separate boots, every algorithm in both
languages agreed to within about 1%, so a whole run on one boot is comparable with another. The
tools in [running.md](running.md) record the machine, OS, compilers and C++ standard library in
every CSV.

## One process per algorithm

`python/bench` times each algorithm in a fresh process, so that none inherits a heap shaped by
another. Within a process, each algorithm still times many points in turn, so its allocations can
land in different places from one point to the next; the alignment fixes below remove the effects
of that we found.

## Alignment

`malloc` only promises 16-byte alignment, so whether an array starts on a 64-byte cache line
depends on what was allocated before it. Where an algorithm sweeps an array with vector loads and
stores, a misaligned start makes about half of them straddle two cache lines.

- SELECT fills an array of all _n_ values on every call. In Rust, filling it took 1.5 times as long
  when it started 16 bytes past a 32-byte boundary. Both implementations now align it to a cache
  line; see [SELECT](algorithms.md#select).
- The quadratic algorithms scan their own output on every draw. Rust quadratic rejection, sorted,
  at _n_ = 1,346,269, _k_ = 10,946, took either 145 or 178 ns per number, depending on the process;
  C++ quadratic F2, random, at _k_ = 121,393, took 1,897 ns every time, so its allocations were
  always misaligned in the same way. Both harnesses now align the output array they time each
  algorithm with, and these take 112 and 1,290 ns, the same in every process.

## The random number generator

The C++ standard library's random number generation is slow enough that it would dominate every
algorithm's time, so the C++ uses its own: Xoshiro256++ with Lemire's method for a number below a
bound, in [`cpp/randbelow.h`](cpp/randbelow.h). It is defined in the header so that it is inlined
into each algorithm, and it takes two 32-bit numbers from each 64-bit output. The Rust uses the
`rand` crate's `SmallRng`, which is the same generator, and its `random_range`, which takes a whole
64-bit output for each 32-bit number; this is most of why the shuffle that the random-order
variants end with costs more in Rust (see [the results](results.md)).

Both use the multiplication trick to turn a random 32-bit number into one below a bound, but they
correct its slight bias differently. Lemire's method rejects the few inputs that cause it and draws
again, so it is exact. `rand`'s default, Canon's method, instead adds in a second multiplication
when the first is suspect and never loops, which leaves a bias of at most 1 in 2<sup>96</sup> for
32-bit numbers. `rand` has an `unbiased` feature that makes it exact, at a cost on every draw; we
don't enable it, since that cost would favour the algorithms that draw fewer numbers, and a bias
that small can't be detected.

## Inlining in Rust

`rand`'s `random_range` is not marked `#[inline]`, so whether a draw is inlined is up to LLVM's
heuristics. With an inclusive range, as in `random_range(0..=i)`, LLVM kept it out of line once it
had several callers, and every draw became a call that passed the generator's state through memory.
Sorted reservoir sampling at _n_ = 1,346,269, _k_ = 144 took 1.73 ns per value of _n_ that way,
against 0.90 ns inlined, and 0.91 ns in C++. The Rust now draws from `0..x + 1` instead, which LLVM
inlines everywhere; adding `#[inline]` to `random_range` in a patched copy of `rand` had the same
effect.

The same kind of thing showed up when the Rust moved from a timing binary of its own into the
library that `python/bench` loads: rejection sampling, Floyd's F2 and HSEL got 14–23% faster. The
old binary also hashed strings and pairs for its command line and CSVs, and with those extra
callers, LLVM kept the hash function out of line; the library inlines it.

## Searching an array in C++

The quadratic algorithms ask on every draw whether a number is already in the output so far. The
C++ first did this with a loop that returned as soon as an entry matched, which clang did not
vectorise because of the early exit. It now uses a loop that ORs every comparison into a `bool`,
in [`cpp/contains.h`](cpp/contains.h), which clang vectorises. The Rust uses its standard library's
`slice::contains`.

We also tried libc++'s `std::ranges::contains`, which on long arrays scans almost twice as fast as
our loop. But for 4-byte integers libc++ makes it a call to glibc's `wmemchr`, and the call costs
more than a short scan does. The quadratic algorithms are only worth having at small _k_, where
the scans are short, so we kept our loop. In ns per number, for C++ quadratic rejection, random
order, at _n_ = 1,346,269:

| _k_ | Returning early | ORing into a `bool` | `std::ranges::contains` |
|---|---|---|---|
| 8 | 2.48 | 1.86 | 5.04 |
| 34 | 8.35 | 2.65 | 5.24 |
| 144 | 38.3 | 3.96 | 5.28 |
| 610 | 153 | 10.2 | 7.39 |
| 10,946 | 2,483 | 169 | 91.7 |

## The C++ standard library

On Linux, clang uses GCC's libstdc++ by default. The C++ is built with clang's own libc++ on Linux
as on macOS, since libstdc++'s `std::sort` and hash tables are different code: switching to libc++
made Cardchoose take about 0.55 times as long, the other sorted variants about 0.8 times, and HSEL
about 1.1 times.

## Large allocations on Linux

glibc's `malloc` serves blocks over 32 MB with a fresh `mmap` and unmaps them on `free`, so an
algorithm that allocates a large array on every call faults in every page of it again on every
call. Of the algorithms here, only SELECT does, from _n_ ≈ 8 million; see
[SELECT](algorithms.md#select). The `faults` rows in the results CSVs record the page faults each
point took.

## Running unattended

The benchmark runs as a systemd unit on the machine, outside any login session. When the last SSH
session ended, logind removed the bench user's IPC objects, including the semaphores that Python's
process pool was using; the bench user now lingers, so logind leaves them alone.

## How we checked

- Every change to an algorithm is checked for fairness with `python/test_choose`, which counts how
  often each possible output comes up.
- To tell a real change from noise, the old and new code are timed alternately, on one boot, at the
  same points.
- To find why one version was faster, we read the machine code of both, on the machine the results
  are timed on.
