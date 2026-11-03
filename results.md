# Results

These results were measured on an AWS r8a.xlarge, whose four vCPUs are each a whole core of an AMD
EPYC 9R45, with 32GiB of memory, running NixOS 25.11, using clang 21.1 with libc++ and rustc 1.98,
for values of _n_ and _k_ that are Fibonacci numbers, with _k_ at most 102,334,155 (the 40th
Fibonacci number). [Full results](results/latest/)

Each algorithm is timed in a process of its own, and its CSVs record the commit and setup it was
timed with.

Since both _n_ and _k_ affect the performance, I compare algorithms using an animated graph. For
each frame, I fix the value of _n_ and graph the performance of each algorithm for various values
of _k_; I graph time/_k_ to show the cost in ns per number output. Since the graph of time/_k_ is
linear/log, the _k_ log _k_ performance of `cardchoose` is visible as a straight line.

I stop timing an algorithm at larger values of _k_ for the same _n_ once a single call takes more
than 30 seconds, and at that _k_ and all larger ones for every later _n_ once that has happened at
the same _k_ for two values of _n_. Only the quadratic algorithms went over the limit, from _k_ =
1.3×10<sup>6</sup>.

## Comparing algorithms

[Choosing algorithms](choosing.md) scores the algorithms by their worst time per number output in
each decade of _k_, gives the best algorithm in each band and the best sets of two or three to
implement together, and makes the case for implementing only `cardchoose`.
[Every algorithm's score in every band](results/latest/by-band.md) gives the full detail.

The region claims below come from [`python/regions`](python/regions), which gives the range of
one algorithm's time over another's, or over the fastest of all the others, in bands of _n_/_k_
and _k_.

## Each pair of algorithms

For every pair of algorithms, the pairwise comparisons give how much slower one is than the other
at its worst, and where, both as a [ratio of times](results/latest/pairwise-ratio.md) and as a
[difference in time/_k_](results/latest/pairwise-difference.md).

## C++

The C++ implementations use only the standard library, here libc++: `std::unordered_set` and
`std::unordered_map`, which are node-based, and `std::sort`. So these results show the
performance you get from standard components.

For sorted output, `hiddenshuffle` is the fastest algorithm wherever _n_ ≥ 16 _k_ and _k_ ≥ 1000.
There, `cardchoose` takes up to 2.4× as long, and `ranksb` 1.14–2.7× as long. `ranksb` slows down as
_k_ approaches _n_, where Nijenhuis and Wilf pair it with `iterativechoose` (which they refer to as
RKS2). For random output, `select` is the fastest wherever 2 _k_ ≤ _n_ < 16 _k_ and _n_ is under 8
million; for larger _n_, where each call maps its array afresh (see [SELECT](algorithms.md#select)),
it takes up to 1.8× as long as the fastest there. With _k_ ≥ 1000, `hiddenshuffle` followed by a
shuffle is the fastest where _n_ ≥ 256 _k_. `cardchoose` is faster than `rejectionsample`, `floydf2`
and `hsel` at every point where _k_ ≤ _n_/2: they take 1.12–8.3× as long, and at least 1.15× as
long where _n_ ≥ 4 _k_.

![graph](results/latest/graphs/cpp-random.webp)

![graph](results/latest/graphs/cpp-sorted.webp)

## Rust

The Rust implementations use Rust's standard `HashMap`, `HashSet` and unstable sort. The hash
tables are flat open-addressing tables, much faster than the node-based C++ ones; the hash-based
algorithms take about 0.3× as long in Rust as in C++, and `cardchoose` about 0.5–0.6×, while
`ranksb`, `vitterd` and `hiddenshuffle` take about the same time in both for sorted output, and
about 10% longer in Rust for random output. The difference is in the shuffle that turns their sorted
output into random order, which costs up to about twice as much per number in Rust. It is the same
loop in both languages, but the Rust draws each index with the `rand` crate's `random_range`,
while the C++ uses its own, which takes two indices from each 64-bit random number. At large _k_ the shuffle is close to half of `cardchoose`'s time in Rust, against a
quarter in C++. (The `rand` crate's own `SliceRandom::shuffle` is slower still.)

For sorted output, `cardchoose` and `hiddenshuffle` divide the range by _k_. Wherever _k_ < 1000 and
_n_ ≥ 2 _k_, `cardchoose` is the fastest algorithm or within 22% of it, and `hiddenshuffle` takes up
to 3.7× as long as `cardchoose`. For larger _k_ they trade places: wherever _k_ ≥ 10<sup>5</sup> and
_n_ ≥ 16 _k_, each is within 26% of the fastest algorithm, and `hiddenshuffle` is the fastest
wherever _k_ ≥ 10<sup>7</sup> and _n_ ≥ 16 _k_. For random output, as in C++, `select` is the
fastest wherever 2 _k_ ≤ _n_ < 16 _k_ and _n_ is under 8 million, and `hsel` is within 16% of the
fastest wherever _n_ ≥ 256 _k_ and 1000 ≤ _k_ < 10<sup>5</sup>. `cardchoose` is never more than
1.57× slower than the fastest of `rejectionsample`, `floydf2`, `hsel` and itself where _k_ ≤ _n_/2.

![graph](results/latest/graphs/rust-random.webp)

![graph](results/latest/graphs/rust-sorted.webp)
