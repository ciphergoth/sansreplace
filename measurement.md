# Making the timings trustworthy

Again and again, a result that looked odd, such as one algorithm faster than another in one
language but slower in the other, or a point that changed between runs, turned out to come from how
it was measured rather than from the algorithm. Most of the effects below moved some algorithm's
time by 10% or more, and some by far more, while the algorithm's code stayed the same: often more
than the gap between two algorithms that we would otherwise have reported as a finding. This page
lists what we found and what we changed, so that the [results](results.md) measure the algorithms
and the libraries they use, and not accidents of code layout, memory layout or code generation.
The times quoted here are from the AWS r8a.xlarge that the published results were timed on.

## The machine

The results are timed on an AWS r8a.xlarge, whose four vCPUs are each a whole core of an AMD EPYC
9R45, so nothing else shares the core a benchmark runs on. It is stopped between runs, and may come
back on a different host. One of the four cores is kept for the timing: the kernel is booted with
`isolcpus=3 nohz_full=3 rcu_nocbs=3`, so that it schedules nothing else there, sends it no timer
ticks while one process runs, and does its own housekeeping on the other cores, and `python/bench`
pins every process it times to that core. Transparent huge pages are off. The tools in
[running.md](running.md) record the machine, OS, kernel command line, compilers, C++ standard
library and Boost version in every CSV.

It is a virtual machine, which limits what we can look at: it offers five performance counters at
a time, and no way to turn the processor's prefetchers off.

## How a point is timed

Each point, one algorithm at one _n_ and _k_, is timed in a process of its own, which
`python/bench` starts afresh for it. In the earlier harness one process timed all of an algorithm's
points in turn, and a point's time then depended on what had been timed before it: on where earlier
points had left the heap, and on states of the processor that
[outlasted the point that set them off](#the-quadratic-scans-slow-state). A fresh process starts
every point from the same state, so a point timed alone gets the time it gets in a whole run.

The process times batches of calls. It doubles the calls in a batch until a batch takes a tenth of
`--min-time`, then times batches of that size until it has at least three, over at least
`--min-time`, 0.05 seconds, and reports the time per call of the fastest batch. Every batch starts
from the same seed, so every batch does the same work, and the fastest is the one the machine
disturbed least. A point whose first call takes a second or more is timed by that call alone.

It does this at up to four [positions of the stack](#where-the-stack-lands), and the point's time
is the fastest of them. The CSVs keep every position's fastest and median batch, the minor page
faults it took, and with `--counters`, five performance counters over the fastest batch: cycles,
instructions, branch mispredictions, requests from the level-2 prefetcher and level-2 cache misses.
The instruction count is the same to six figures from one run to the next, so when a time moves,
the counters say whether the processor did the same work more slowly or did different work.

## Where the stack lands

Where a process's stack starts depends on things that have nothing to do with the program. The
kernel randomises it, with the rest of the address space, on every launch; and the environment
variables sit above it, so that its start moves with their total length. Mytkowicz et al., in
[Producing wrong data without doing anything obviously
wrong!](https://doi.org/10.1145/1508244.1508275), found the size of the environment changing which
of two builds looked faster, and it does here too:

- With the address space randomised as usual, 20 processes each timing C++ HSEL at _k_ = 1 spread
  over a factor of 1.72, and only 11 of them came within 2% of the fastest. With randomisation off,
  all 20 did.
- With randomisation off, and the environment padded by 0 to 4,080 bytes in steps of 16, the same
  point's time moved between 0% and 6% above its fastest in a pattern that repeats every 64 bytes,
  so with the stack's position within a cache line; and at a few positions it took 1.3 to 1.7
  times as long. An empty call took 8.4 times as long at one position of the 256.

So `bench` starts every process with randomisation off, through `setarch -R`, and with an empty
environment, and each process runs the timing on a stack it allocates itself, at a place it
chooses: 0, 16, 32 and 48 bytes below a page boundary in turn, which covers the cycle. `bench`
takes the fastest of the four. This gives every algorithm its best alignment rather than whichever
one the launch happened to give it; a point whose single call takes `--min-time` or more is timed
at the first position only, since at that size the others would cost more than they could tell us.

## The random number generator

The C++ standard library's random number generation is slow enough that it would dominate every
algorithm's time, so the C++ uses its own: Xoshiro256++ with Lemire's method for a number below a
bound, in [`cpp/randbelow.h`](cpp/randbelow.h). It is defined in the header so that it is inlined
into each algorithm, and it takes two 32-bit numbers from each 64-bit output. The Rust uses the
`rand` crate's `SmallRng`, which is the same generator, and its `random_range`, which takes a whole
64-bit output for each 32-bit number. Both generators, and the Python's, start each batch that
times a point from a seed fixed by that point, so that every batch makes the same draws, in any
run, and the C++ and Rust generators start each batch in the same state. The hash tables in both
languages also hash with a fixed seed, so a batch at a given point does exactly the same work
every time.

Both use the multiplication trick to turn a random 32-bit number into one below a bound, but they
correct its slight bias differently. Lemire's method rejects the few inputs that cause it and draws
again, so it is exact. `rand`'s default, Canon's method, instead adds in a second multiplication
when the first is suspect and never loops, which leaves a bias of at most 1 in 2<sup>96</sup> for
32-bit numbers. `rand` has an `unbiased` feature that makes it exact, at a cost on every draw; we
don't enable it, since that cost would favour the algorithms that draw fewer numbers, and a bias
that small can't be detected.

## Where code lands

Where a function starts within its 64-byte cache line depends on all the code the linker put
before it, and that alone moved the times of tight loops whose code had not changed:

- C++ iterative choosing, random order, took 12–15% longer, with identical machine code, when the
  C++ standard library changed and moved the code before it.
- The Rust bit-vector F2, sorted, took up to 1.7 times as long at large _n_/_k_, where its time goes
  on scanning the bit vector, when the Rust became one codegen unit (below).

Both languages now start every function on a cache line, with `-falign-functions=64` in
[`cpp/meson.build`](cpp/meson.build) and `-align-all-functions=6` in
[`.cargo/config.toml`](.cargo/config.toml), and both of these went back to their earlier times.
This makes the timings stable when unrelated code changes, but not optimal: a loop's alignment
still depends on the code before it in its own function.

## Where data lands

`malloc` only promises 16-byte alignment, so whether an array starts on a 64-byte cache line
depends on what was allocated before it. Where an algorithm sweeps an array with vector loads and
stores, a misaligned start makes about half of them straddle two cache lines.

- SELECT fills an array of all _n_ values on every call. In Rust, filling it took 1.5 times as long
  when it started 16 bytes past a 32-byte boundary. Both implementations now align it to a cache
  line; see [SELECT](algorithms.md#select).
- The quadratic algorithms scan their own output on every draw. C++ quadratic F2, random, at
  _k_ = 121,393, took 1,897 ns per number in every process, so its allocations were always
  misaligned in the same way. Both programs now align the output array they time each algorithm
  with, and it takes 1,290 ns.

## What the compiler decides

The same source can compile to very different code depending on things outside it.

### Inlining in Rust

`rand`'s `random_range` is not marked `#[inline]`, so whether a draw is inlined is up to LLVM's
heuristics. With an inclusive range, as in `random_range(0..=i)`, LLVM kept it out of line once it
had several callers, and every draw became a call that passed the generator's state through memory.
Sorted reservoir sampling at _n_ = 1,346,269, _k_ = 144 took 1.73 ns per value of _n_ that way,
against 0.90 ns inlined, and 0.91 ns in C++. The Rust now draws from `0..x + 1` instead, which LLVM
inlines everywhere; adding `#[inline]` to `random_range` in a patched copy of `rand` had the same
effect.

The same kind of thing showed up in an early timing program for the Rust, which also hashed
strings and pairs for its command line and CSVs: with those extra callers, LLVM kept the hash
function out of line, and rejection sampling, Floyd's F2 and HSEL took 14–23% longer than they do
now. Today's program uses no hash table outside the algorithms, so the hash function is inlined
into them.

### Codegen units in Rust

By default, Cargo's release profile splits a crate into 16 codegen units, placing each function,
and each copy of generic code such as the standard library's sort for `u32`, by heuristics over
the whole crate, and LLVM decides what to inline within each unit separately. So when we changed
the hash tables' hasher, code that has no hash table compiled differently: the standard library's
sort inlined its small-sort routine and took 7–16% longer, several algorithms were or were not
inlined into their callers, and the time of an empty call went from 0.90 to 1.11 ns. The Rust is
now built as one codegen unit with link-time optimisation, in [`rust/Cargo.toml`](rust/Cargo.toml),
so that what one function compiles to depends far less on the rest of the crate; with it, the sort
went back to its earlier speed, and random-order reservoir sampling became about 19% faster.

### Searching an array in C++

The quadratic algorithms ask on every draw whether a number is already in the output so far. The
C++ first did this with a loop that returned as soon as an entry matched, which clang did not
vectorise because of the early exit. It now uses a loop that ORs every comparison into a `bool`,
in [`cpp/contains.h`](cpp/contains.h), which clang vectorises. The Rust uses its standard library's
`slice::contains`, which checks 16 entries at a time before testing for a match.

When the C++ was built with libc++, we also tried its `std::ranges::contains`, which on long arrays
scans almost twice as fast as our loop. But for 4-byte integers libc++ makes it a call to glibc's
`wmemchr`, and the call costs more than a short scan does. The quadratic algorithms are only worth
having at small _k_, where the scans are short, so we kept our loop. In ns per number, for C++
quadratic rejection, random order, at _n_ = 1,346,269, with libc++:

| _k_ | Returning early | ORing into a `bool` | `std::ranges::contains` |
|---|---|---|---|
| 8 | 2.48 | 1.86 | 5.04 |
| 34 | 8.35 | 2.65 | 5.24 |
| 144 | 38.3 | 3.96 | 5.28 |
| 610 | 153 | 10.2 | 7.39 |
| 10,946 | 2,483 | 169 | 91.7 |

### Defaults that don't suit a benchmark

We build differently from the toolchains' defaults in four ways. To see what three of them are
worth, we timed every algorithm in both orders, and the components, at _n_ = 1,346,269 and
_k_ = 34, 610, 10,946 and 196,418, with each default restored in turn, taking each point's fastest
of three interleaved runs.

- **The target.** On x86-64, both compilers by default target the original 2003 baseline, whose
  only vectors are SSE2. Both languages are built for x86-64-v3, the AVX2 baseline, which every
  x86-64 server of the last decade supports. This is the one that matters: built for the old
  baseline, the quadratic algorithms take 1.7 to 3.9 times as long from _k_ = 610 up in both
  languages, since their scans then use vectors half as wide; SELECT's fill takes up
  to 1.2 times as long; and the C++ sorted bit-vector F2 takes 1.4 to 1.8 times as long at small
  _k_, where its time goes on scanning the bit vector for set bits. Most other algorithms are within
  a few percent, and C++ iterative choosing is 8% faster.
- **Nix's hardening.** Nix's compiler wrapper adds hardening flags to every compile, among them
  stack protectors, `-fwrapv`, `_FORTIFY_SOURCE` and zeroing registers on return. An ordinary build
  doesn't get these, so [`devenv.nix`](devenv.nix) turns them all off. With them, every C++ time
  was within 5% of its time without, except the sorted bit-vector F2 at _k_ = 34, at 7% slower.
- **Meson's assertions.** With `b_ndebug` unset, Meson turns on the C++ standard library's own
  checks, `_GLIBCXX_ASSERTIONS` for libstdc++, which an ordinary release build doesn't have. Now
  that the C++ takes its sort and hash tables from Boost, they make little difference: every C++
  time was within 4% of its time without, except random-order SELECT, which was 9% faster with
  them.
- **The compiler.** On Linux, Nix's default compiler is GCC. Both languages are built with LLVM,
  with clang for the C++, so that the two differ in their libraries and not their compilers.

## The libraries

Which library provides a sort or a hash table changes the results more than most of the effects
above, and can decide which language looks faster.

- **Hash tables.** The C++ standard library's `std::unordered_set` and `std::unordered_map` are
  node-based: each entry is allocated separately and reached through a linked list. The C++ now
  uses Boost's `unordered_flat_set` and `unordered_flat_map`, flat open-addressing tables like
  Rust's `HashSet` and `HashMap`; with them, inserting into a hash set alone is 4–16 times as fast,
  and the C++ Floyd's F2, HSEL and set-based rejection sampling take about 0.2 times as long in
  random order and 0.45 times as long sorted.
- **Hashing.** Rust's tables hash with SipHash by default, which resists collision attacks; Boost's
  hash does not, so the Rust hashes with foldhash instead, with a fixed seed. That made inserting
  into a hash set alone about 1.6 times as fast, and the Rust's hash-based algorithms take 0.6–0.8
  times as long.
- **Sorting.** The C++ uses Boost's `pdqsort`. libstdc++'s `std::sort` and libc++'s are different
  code: switching from libstdc++ to libc++ made Cardchoose take about 0.55 times as long, and the
  other sorted variants about 0.8 times. Rust's `sort_unstable`, ipnsort, still sorts in about half
  the time that `pdqsort` takes.

## Memory from the kernel

glibc's `malloc` serves blocks over 32 MB with a fresh `mmap` and unmaps them on `free`, so an
algorithm that allocates a large array on every call faults in every page of it again on every
call. Of the algorithms here, SELECT does, from _n_ ≈ 8 million, and the bit-vector F2 from
_n_ ≈ 270 million; see [SELECT](algorithms.md#select). The `position` rows in the results CSVs
record the page faults each point took.

The results are timed with ordinary 4 KB pages, which is what a program gets unless it asks for
something else. `bench --huge-pages` has glibc take its memory in 2 MB pages instead, and we timed
20 points both ways to see what that changes:

- The algorithms whose time goes on cache misses in a large table get faster, since each miss
  no longer also misses in the TLB: by 5% for Rust rank-select at _n_ = _k_ = 3,524,578, 11% for
  Rust sorted rejection sampling at _k_ = 5,702,887 and 17% for the C++ bit-vector F2 at
  _n_ = 165,580,141.
- SELECT at _n_ = 14,930,352, which faults its array in on every call, takes 0.22 times as long,
  since 14,600 faults a call become 29.
- SELECT at _n_ = 1,346,269 takes twice as long. With 2 MB pages glibc maps even a 5 MB array
  afresh on every call, and the kernel zeroes whole 2 MB pages for it.
- Everything else we tried is within the noise.

So the page size changes which algorithm is fastest at some points, in both directions, and we
report the default.

## The prefetcher's two speeds

Filling 185 KB with the numbers from 0 to 46,367, a loop that executes exactly the same 16,209
instructions every time, runs at two speeds 1.3 times apart: 3,370 cycles or 4,255. Nothing we can
count differs between them in cache misses, TLB misses or branch mispredictions. What differs is
the level-2 cache's prefetcher, which makes between one and two requests during a fast fill and
about 1,160 during a slow one. The array fits in the level-2 cache and is already there, so the
requests fetch nothing that helps, and the fill is slower while they are made. This is a
correlation: the virtual machine gives us no way to turn the prefetcher off and see the slow speed
go away.

Which speed a process gets depends on where its memory lies. With 4 KB pages the fill is fast at
some positions of the stack and slow at others, and occasionally a process finds no fast position
among its four; with 2 MB pages it is always slow. We have not found the rule. The `count`
component, which is this fill and nothing else, is timed with the others so that the results show
it: SELECT, which begins with the same fill, runs at the slow speed at _n_ = 46,368 in both
languages, in every process, with the same 1,250 or so prefetcher requests a call.

This is the largest effect on this page that we can't remove, and it is a property of the
processor rather than of our measurement: a program that calls SELECT on this machine will get one
speed or the other for reasons it can't see. Where a point's positions differ by more than a few
percent, the `position` rows in the CSV show both speeds.

## The quadratic scan's slow state

Rust quadratic rejection sampling, and sorted quadratic F2, which both scan with
`slice::contains`, run at two speeds. In the earlier harness, where one process timed all of an
algorithm's points, a whole row of points at large _k_, or a stretch of one, sometimes took about
1.6 times as long as in other runs, and which points were slow changed from run to run. The code,
the data and the draws are the same in both: at _k_ = 6,765, the fast state
takes 70.7 ns per number. Hardware performance counters show what differs:

| | Fast | Slow |
|---|---|---|
| Cycles per iteration of the scan, 16 entries | 1.5 | 2.3 |
| Operations retired per cycle | 4.5 | 3.0 |
| Dispatch slots (of 8) empty for want of operations from the front end | 2.1 | 4.8 |
| Dispatch slots (of 8) empty because the back end is stalled | 0.75 | about 0 |

So in the slow state, the front end of the core, which fetches instructions and turns them into
operations, cannot keep up, and the back end, which executes them, waits for it. In both states,
every operation comes from the op cache, with no misses, and there are no more branch
mispredictions, cache misses or TLB misses in one than the other.

We have ruled out the other core of an SMT pair (there is none), moving between cores, transparent
huge pages, where the output array starts, the data, the host (another process timing the same
point on another core at the same time stayed fast), and the passage of time (one point timed for
six minutes stayed fast). We have not found what triggers it, but where the process's memory lies
has a part in it: of 20 fresh processes that each timed only the point at _k_ = 6,765, with the
address space randomised, one was in the slow state, and of 20 with randomisation off, none was.
Nor was any of the ten processes that have timed that point with today's harness, though one of
20 was while we were developing it, with 2 MB pages. So timing each point in its own process has
made the slow state rare, and has kept one slow point from slowing the points after it, but we
can't say it is gone.

## How far to trust a time

To see how much a time changes from one run of `bench` to the next, we timed 20 points five times
each, among them every point above that had given trouble. With one exception, the five times of
every point are within 3% of each other, and of half of them within 1%. The widest are C++ Floyd's
F2 in random order at _n_ = _k_ = 1,346,269, at 3.0%, C++ HSEL at _k_ = 1, at 2.8%, and C++ SELECT
at _n_ = 14,930,352, at 2.4%. The exception is `count` at _k_ = 46,368, for the
[prefetcher's](#the-prefetchers-two-speeds) reasons: four processes in ten found only the slow
speed.

So we would not read anything into a difference of less than 5% between two algorithms at a point,
and where an algorithm's time goes on sweeping an array of a few hundred kilobytes, the
prefetcher's two speeds can move it by up to 1.3 times.

## How we checked

- Every change to an algorithm is checked for fairness with `python/test_choose`, which counts how
  often each possible output comes up, and `python/test_all` does the same for every algorithm in
  both orders and every language at a few small points.
- A time that changes unexpectedly is timed again, on a fresh boot, before we believe it.
- To tell a real change from noise, the old and new code are built and timed alternately, on one
  boot, at the same points.
- When a time changes but the source hasn't, we compare the machine code of both builds, on the
  machine the results are timed on, so that "the code is the same" is something we have checked
  rather than assumed.
- Where the machine code is the same, we read the processor's performance counters to see where the
  time goes; `bench --counters` records five of them with every point.
- A point that gives trouble is timed in many fresh processes, with one thing changed at a time,
  such as the randomisation, the environment or the page size.
