# Algorithms for sampling without replacement

In the standard form of the problem, the answers can be in any order and
so the order must also be chosen fairly; there is a variation in which the answers must be in sorted
order.  Any algorithm can be turned into one with a "sorted" order guarantee
with an O(_k_ log _k_) sort, or a "random" order guarantee with an O(_k_)
Fisher-Yates shuffle.

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | [Rust](rust/src/algorithms/quadraticreject.rs) | Random | none | _k_^2
Set-based rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | [Rust](rust/src/algorithms/rejectionsample.rs) | Random | Set | _k_
RANKSB | [Python](python/algorithms/ranksb.py) | [C++](cpp/ranksb.cpp) | [Rust](rust/src/algorithms/ranksb.rs) | Sorted | none | _k_
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | [Rust](rust/src/algorithms/floydf2.rs) | none | Set | _k_
Quadratic F2 | [Python](python/algorithms/quadraticf2.py) | [C++](cpp/quadraticf2.cpp) | [Rust](rust/src/algorithms/quadraticf2.rs) | Random | none | _k_^2
Iterative choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | [Rust](rust/src/algorithms/iterativechoose.rs) | Sorted | none | _n_
Reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | [Rust](rust/src/algorithms/reservoirsample.rs) | Random | none | _n_
Vitter's Algorithm D | [Python](python/algorithms/vitterd.py) | [C++](cpp/vitterd.cpp) | [Rust](rust/src/algorithms/vitterd.rs) | Sorted | none | _k_
Hidden Shuffle | [Python](python/algorithms/hiddenshuffle.py) | [C++](cpp/hiddenshuffle.cpp) | [Rust](rust/src/algorithms/hiddenshuffle.rs) | Sorted | none | _k_
SELECT | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | [Rust](rust/src/algorithms/select.rs) | Random | n-sized list | _n_
HSEL | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | [Rust](rust/src/algorithms/hsel.rs) | Random | Dictionary | _k_
[Cardchoose](cardchoose.md) | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | [Rust](rust/src/algorithms/cardchoose.rs) | Sorted | none | _k_ log _k_

## Quadratic rejection sampling

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | [Rust](rust/src/algorithms/quadraticreject.rs) | Random | none | _k_^2

For each output, try choosing
an integer in [0, _n_), but check every integer already in the output to see if
it's the same, and reject and re-draw if so. This is quadratic in _k_, but for small
_k_ the small constants can mean this algorithm does very well. It takes O(_k_^2) expected time for
_k_ ≤ _n_/2, and more as _k_ approaches _n_ and more draws are rejected, as for set-based rejection
sampling.

The check is the inner loop, so it matters that the compiler can vectorise it, comparing several
entries in one instruction. My C++ checks every entry rather than stopping at a match, since a
loop that can stop early doesn't vectorise. My Rust uses the standard library's `contains`, which
gets both: it compares entries 16 at a time with vector instructions, and stops after the first
block of 16 that has a match.

## Set-based rejection sampling

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Set-based rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | [Rust](rust/src/algorithms/rejectionsample.rs) | Random | Set | _k_

Maintain a set of already-produced values. While the output isn't full,
try choosing a random integer in [0, _n_). If this is in the set of already-produced
values, discard it. Otherwise, append it to the output, and add it to the list of
already-produced values.

One of the simplest possible algorithms, but the cost of set lookups and insertions
is high. It takes O(_k_) expected time for _k_ ≤ _n_/2, where each draw is rejected with
probability at most a half. As _k_ approaches _n_, rejections mount: the expected number of draws
is _n_(_H_<sub>_n_</sub> − _H_<sub>_n_−_k_</sub>), where _H_ is the harmonic number, so at
_k_ = _n_ it takes O(_n_ log _n_) time.

## RANKSB

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
RANKSB | [Python](python/algorithms/ranksb.py) | [C++](cpp/ranksb.cpp) | [Rust](rust/src/algorithms/ranksb.rs) | Sorted | none | _k_

Divide [0, _n_) into _k_ bins of nearly equal size. First decide how many of the chosen values
fall in each bin, by simulating set-based rejection sampling while keeping only a count for each
bin: a draw is rejected with the probability that it would duplicate a value already accepted in
its bin, and that depends only on how many values the bin has accepted. Then choose that many
values within each bin; bins hold about one value each, so a quadratic method is fine. The
output comes out sorted, it takes O(_k_) expected time for _k_ ≤ _n_/2, and the book's program
needs no storage beyond the output array. As _k_ approaches _n_ it slows down, and for
_k_ ≥ _n_/2 the book pairs it with iterative choosing (which Nijenhuis and Wilf refer to as
RKS2); I benchmark RANKSB on its own.

My C++ and Rust implementations follow the program in the book, except that they fill each bin
as soon as they know where its values go, rather than first marking out every bin's slots and
then scanning for the marks; the Python one keeps the counts in a separate array, which is easier
to follow.

* Albert Nijenhuis and Herbert S. Wilf
    * Combinatorial Algorithms for Computers and Calculators, 2nd edition
    * Academic Press, 1978
    * Chapter 4, "Random _k_-Subset of an _n_-Set (RANKSB)"

## Floyd's F2

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | [Rust](rust/src/algorithms/floydf2.rs) | none | Set | _k_

I refer you to two excellent explanations:

* [A Sample of Brilliance](https://blog.acolyer.org/2018/01/30/a-sample-of-brilliance/)
* [Robert Floyd's Tiny and Beautiful Algorithm for Sampling without Replacement](http://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html)

The original presentation is as one of Jon Bentley's Programming Pearls, though the algorithm is due to Robert W Floyd: [A Sample of Brilliance](https://doi.org/10.1145/30401.315746).

## Quadratic F2

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Quadratic F2 | [Python](python/algorithms/quadraticf2.py) | [C++](cpp/quadraticf2.cpp) | [Rust](rust/src/algorithms/quadraticf2.rs) | Random | none | _k_^2

A quadratic variant on Floyd's F2 which needs no external data structure: the output array is the
set, scanned to check each draw. When the draw _r_ is already present, it replaces that entry with
_m_ and appends _r_, which keeps the output in uniformly random order without a shuffle. The
[Rust rand crate](https://github.com/rust-random/rand/blob/19897e53c1908c5a193e2aacff59170e3e72b8de/src/seq/index.rs#L231)
used Floyd's F2 with a scan in the same way, but kept the order random either by inserting _m_
before the matching entry, which shifts every entry after it, or by shuffling the whole output
afterwards; it has used this version since
[rand pull request 1277](https://github.com/rust-random/rand/pull/1277), in 2023. Like quadratic
rejection sampling, the small constants mean this does very well for small _k_.

For random order, each step draws _t_ and replaces the existing _t_, if there is one, with _j_.
At most one entry matches, so my implementations write every entry, keeping its value unless it
matches, rather than stopping at the match; a loop that can stop early doesn't vectorise. For
sorted order, only the set matters until the final sort, so where _t_ is already present, a step
adds _j_ instead of moving the existing _t_ to _j_. That gives the same set, and the step only has
to check whether _t_ is present, as in quadratic rejection sampling, which is faster than
rewriting every entry.

## Iterative choosing (Algorithm S)

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Iterative choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | [Rust](rust/src/algorithms/iterativechoose.rs) | Sorted | none | _n_

Iterate through every possible value in [0, _n_) in order and decide whether to include it in the
output, weighting the probability appropriately depending on the number of spaces left in the output
and the number of values remaining to test. One of the faster algorithms for producing sorted output
where _k_ is a large fraction of _n_.

* Donald Knuth
    * The Art of Computer Programming
    * Volume 2: Seminumerical Algorithms.
    * Section 3.4.2
    * Page 137
    * "Algorithm S"

## Reservoir sampling (Algorithm R)

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | [Rust](rust/src/algorithms/reservoirsample.rs) | Random | none | _n_

Initialize the output with the integers [0, _k_) in random order.
Then iterate through the remaining values [k, _n_) in order and decide
whether to replace an existing, randomly chosen output with that integer.
Fast for producing random output where _k_ is a large fraction of _n_.

* Donald Knuth
    * The Art of Computer Programming
    * Volume 2: Seminumerical Algorithms.
    * Section 3.4.2
    * Page 138
    * "Algorithm R"

## Vitter's Algorithm D

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Vitter's Algorithm D | [Python](python/algorithms/vitterd.py) | [C++](cpp/vitterd.cpp) | [Rust](rust/src/algorithms/vitterd.rs) | Sorted | none | _k_

A sequential sampling algorithm: like iterative choosing, it produces the output in sorted order, but
instead of deciding about each value in [0, _n_) in turn, it draws the number of values to skip
before the next chosen one directly. The skip has a known discrete distribution, and Algorithm D
samples it by rejection from a continuous approximation, the smallest of _k_ uniform reals,
drawn in one step as in Bentley and Saxe's method for generating sorted random numbers. This takes
O(1) expected time per output, so the whole algorithm takes O(_k_) expected time and needs no extra memory. The cost is
floating-point arithmetic: each output needs a few uniform variates and exponentiations. Once
_n_ is at most 13 _k_, it falls back on Vitter's Algorithm A, which draws each skip in O(skip) time,
so the total work for the rest of the output is O(_n_).

My C++ and Rust implementations follow the Pascal-like code in the paper's appendix; the
Python one states the method directly, which is easier to follow.

* Jon Louis Bentley and James B. Saxe
    * Generating Sorted Lists of Random Numbers
    * ACM Transactions on Mathematical Software, Volume 6, Number 3, September 1980
    * 359--364
    * https://doi.org/10.1145/355900.355907

* Jeffrey Scott Vitter
    * An Efficient Algorithm for Sequential Random Sampling
    * ACM Transactions on Mathematical Software, Volume 13, Number 1, March 1987
    * 58--67
    * https://doi.org/10.1145/23002.23003

## Hidden Shuffle

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Hidden Shuffle | [Python](python/algorithms/hiddenshuffle.py) | [C++](cpp/hiddenshuffle.cpp) | [Rust](rust/src/algorithms/hiddenshuffle.rs) | Sorted | none | _k_

Picture a Fisher-Yates shuffle of [0, _n_) that stops after _k_ steps, so the sample is whatever
ends up in the first _k_ positions. Call those the low positions and the rest the high positions.
Each step reaches a high position with a known probability, independently of the others, and which
high position it reaches is uniform and independent of everything else. So first count how many
steps reach a high position. Then draw that many high positions with replacement, in sorted order
as in Bentley and Saxe's method. The first time a high position is drawn, its value joins the
sample. A repeat finds a low value that was swapped there earlier, so the sample gains a low value
instead, much as a collision in Floyd's F2 adds a different value. Given how many values are low,
they are a uniformly random subset of the low positions by symmetry, and Vitter's Algorithm A
chooses them in order. The output comes out sorted, it takes O(_k_) time in the worst case, and it
needs no extra memory. Like Algorithm D, it relies on floating-point arithmetic, including an
exponentiation for each high draw.

The code printed in the paper is wrong: it can divide by zero, and the last step has an off-by-one
error that biases the output. My implementations apply the corrections from the errata in the
authors' repository. Thanks to metasj for bringing this algorithm to my attention.

* Michael Shekelyan and Graham Cormode
    * Sequential Random Sampling Revisited: Hidden Shuffle Method
    * Proceedings of the 24th International Conference on Artificial Intelligence and Statistics
      (AISTATS 2021), PMLR volume 130
    * https://proceedings.mlr.press/v130/shekelyan21a.html
    * Errata and the authors' implementations: https://github.com/shekelyan/sampleiterator

## SELECT

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
SELECT | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | [Rust](rust/src/algorithms/select.rs) | Random | n-sized list | _n_

Initialize an array of the values [0, _n_). Pick out values at random to include
in the output, and delete them by replacing them with a value from one end and
shortening the array.

This is SELECT in Goodman and Hedetniemi's _Introduction to the Design and Analysis of
Algorithms_, though in a slight variation I pick off the array at the start, rather than the end.

Each call allocates the array afresh, so its cost depends on the allocator. The C++ and Rust
implementations align the array to a 64-byte cache line: `malloc` only promises 16 bytes, and on
x86-64, filling the array took 1.5 times as long in Rust when it started 16 bytes past a 32-byte
boundary, which depended on where earlier allocations happened to leave it. On Linux, glibc's
`malloc` serves blocks over 32 MB with a fresh `mmap` and unmaps them on `free`, so from _n_ ≈ 8
million every call faults in every page of the array again; this makes filling it about ten times
slower than on macOS, where nothing like this shows until the largest _n_ timed.

## HSEL

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
HSEL | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | [Rust](rust/src/algorithms/hsel.rs) | Random | Dictionary | _k_

This is a variation of the above which uses a hash table to simulate the array
of values [0, _n_), storing only the entries that change. First proposed by Ernvall and Nevalainen
in 1982:

* Jarmo Ernvall and Olli Nevalainen
    * An algorithm for unbiased random sampling
    * The Computer Journal Volume 25, Number 1, February, 1982
    * 45--47
    * https://academic.oup.com/comjnl/article/25/1/45/527401

Teuhola and Nevalainen give a version that needs 2_k_ storage locations rather than about 5_k_,
which draws all _k_ random numbers first and then works out which value each one stands for, from
the last back:

* J. Teuhola and O. Nevalainen
    * Two efficient algorithms for random sampling without replacement
    * International Journal of Computer Mathematics 11(2), 1982
    * 127--140
    * https://doi.org/10.1080/00207168208803304

It is also presented in Martin Harriman, [Sampling Without Replacement: Durstenfeld-Fisher-Yates
Permutation for Very Large Permutation
Sizes](https://www.tdcommons.org/cgi/viewcontent.cgi?article=1485&context=dpubs_series).

My C++ and Rust implementations use a faster variant, which keeps the first _k_ entries of the
simulated array in the output array itself, so only swaps that reach beyond _k_ touch the hash
table. This variant was described by rici in a [Stack Overflow
answer](https://stackoverflow.com/a/30559195) in 2015. Thanks to Mathias Weller for bringing it to
my attention.

## Cardchoose

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
[Cardchoose](cardchoose.md) | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | [Rust](rust/src/algorithms/cardchoose.rs) | Sorted | none | _k_ log _k_

This is my algorithm: a Pólya urn generates a uniform
multiset, and stars and bars turns it into a uniform subset. [Detailed
description](cardchoose.md), including how it relates to [prior art](priorart.md).
