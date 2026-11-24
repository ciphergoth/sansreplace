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
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | [Rust](rust/src/algorithms/floydf2.rs) | none | Set | _k_
Quadratic F2 | [Python](python/algorithms/quadraticf2.py) | [C++](cpp/quadraticf2.cpp) | [Rust](rust/src/algorithms/quadraticf2.rs) | Random | none | _k_^2
Bit-vector F2 | [Python](python/algorithms/bitvectorf2.py) | [C++](cpp/bitvectorf2.cpp) | [Rust](rust/src/algorithms/bitvectorf2.rs) | Sorted | _n_-bit vector | _n_
Iterative choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | [Rust](rust/src/algorithms/iterativechoose.rs) | Sorted | none | _n_
Reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | [Rust](rust/src/algorithms/reservoirsample.rs) | Random | none | _n_
SELECT | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | [Rust](rust/src/algorithms/select.rs) | Random | n-sized list | _n_
HSEL | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | [Rust](rust/src/algorithms/hsel.rs) | Random | Dictionary | _k_
RANKSB | [Python](python/algorithms/ranksb.py) | [C++](cpp/ranksb.cpp) | [Rust](rust/src/algorithms/ranksb.rs) | Sorted | none | _k_
Vitter's Algorithm D | [Python](python/algorithms/vitterd.py) | [C++](cpp/vitterd.cpp) | [Rust](rust/src/algorithms/vitterd.rs) | Sorted | none | _k_
Hidden Shuffle | [Python](python/algorithms/hiddenshuffle.py) | [C++](cpp/hiddenshuffle.cpp) | [Rust](rust/src/algorithms/hiddenshuffle.rs) | Sorted | none | _k_
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
probability at most a half. As _k_ approaches _n_, rejections mount, as in the coupon collector's
problem: the expected number of draws is _n_(_H_<sub>_n_</sub> − _H_<sub>_n_−_k_</sub>), where _H_
is the harmonic number, so at _k_ = _n_ it takes O(_n_ log _n_) time (Feller, chapter IX §3).

## Floyd's F2

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | [Rust](rust/src/algorithms/floydf2.rs) | none | Set | _k_

I refer you to two excellent explanations:

* [A Sample of Brilliance](https://blog.acolyer.org/2018/01/30/a-sample-of-brilliance/)
* [Robert Floyd's Tiny and Beautiful Algorithm for Sampling without Replacement](http://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html)

F2 returns a set, with no order. For random order, the C++ and Rust implementations do not
shuffle afterwards: as in Daniel Ting's
[random-order version](https://www.tableau.com/blog/fun-optimal-random-sampling), when the chosen
value is already present, they move a randomly chosen earlier output to the end and put the new
value in its place, which costs an extra draw only on collisions. Floyd's own Algorithm P, from
the same article as F2, needs no extra draw, but it has to find the matching entry in the output,
which a set does not record.

The original presentation is as one of Jon Bentley's Programming Pearls, though the algorithm is due to Robert W Floyd: [A Sample of Brilliance](https://doi.org/10.1145/30401.315746).

## Quadratic F2

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Quadratic F2 | [Python](python/algorithms/quadraticf2.py) | [C++](cpp/quadraticf2.cpp) | [Rust](rust/src/algorithms/quadraticf2.rs) | Random | none | _k_^2

A quadratic variant on Floyd's F2 which needs no external data structure: the output array is the
set, scanned to check each draw. For random order, Floyd's Algorithm P, from the same article as
F2, needs no shuffle and no extra draw: a new draw goes on the front of the output, and when the
draw is already present, _m_ is inserted just after it. In an array, that insertion shifts every
entry after it, taking time proportional to their number on each collision. My version instead
replaces the matching entry with _m_ and appends the draw _r_, so that nothing is shifted. As with
Algorithm P, each step can be undone, so every ordered output comes from exactly one sequence of
draws, and all are equally likely. The
[Rust rand crate](https://github.com/rust-random/rand/blob/19897e53c1908c5a193e2aacff59170e3e72b8de/src/seq/index.rs#L231)
also used Floyd's F2 with a scan of the output: for _k_ < 50 it ran Algorithm P in mirror image,
appending each new draw and inserting _m_ before the matching entry, and for larger _k_ it
shuffled the whole output afterwards; it has used this version since
[rand pull request 1277](https://github.com/rust-random/rand/pull/1277), in 2023. Unlike Ting's
random-order Floyd's F2 above, it needs no extra draw on a collision, because the scan has already
found where _r_ is. That pull request is the first publication of my version that I know of. Like quadratic rejection sampling, the small constants mean this does very
well for small _k_.

For random order, each step draws _t_ and replaces the existing _t_, if there is one, with _j_.
At most one entry matches, so my implementations write every entry, keeping its value unless it
matches, rather than stopping at the match; a loop that can stop early doesn't vectorise. For
sorted order, only the set matters until the final sort, so where _t_ is already present, a step
adds _j_ instead of moving the existing _t_ to _j_. That gives the same set, and the step only has
to check whether _t_ is present, as in quadratic rejection sampling, which is faster than
rewriting every entry.

## Bit-vector F2

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
Bit-vector F2 | [Python](python/algorithms/bitvectorf2.py) | [C++](cpp/bitvectorf2.cpp) | [Rust](rust/src/algorithms/bitvectorf2.rs) | Sorted | _n_-bit vector | _n_

Floyd's F2 with a bit vector of _n_ bits as its set: each step tests one bit and sets one. Teuhola
and Nevalainen, and Devroye in Chapter XII §2 of _Non-Uniform Random Variate Generation_, describe
a bit vector as the set for rejection sampling; Bentley times one in Column 13 of _Programming
Pearls_, and sets Floyd's algorithm with the same sets as an exercise. The draws take time proportional to _k_, but
clearing the bit vector takes time proportional to _n_, though little for each value, since a
word covers 64 of them. So it does best where _n_/_k_ is moderate: it is the fastest algorithm
for sorted output wherever 2 _k_ ≤ _n_ < 16 _k_ and _k_ < 10<sup>7</sup>, and for random output
at most points with 16 _k_ ≤ _n_ < 256 _k_; see the [results](results.md).

For sorted order, nothing is written to the output until the end, when one scan of the bit vector
writes out the position of every bit that is set, as Bentley does there and in his bit-vector sort in
Column 1; no sort is needed. For random order there is no scan. The output is filled
in as the values are drawn, and the order is kept random as in Floyd's F2 above: when the draw is
already present, a randomly chosen earlier output moves to the end and _m_ takes its place, which
costs an extra draw. A bit vector says only that a value is present, not where it is in the
output, so quadratic F2's way of doing this without the extra draw isn't open to it.

The Python uses a byte for each bit, for clarity. The C++ and Rust take a packed bit vector from
a library, Boost's `dynamic_bitset` and the `fixedbitset` crate, as they take their sort and hash
tables from libraries. The two differ in how they read the set bits off: see [the
libraries](measurement.md#the-libraries).

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
million every call faults in every page of the array again: with _k_ = 34, a call takes 0.034 ns
for each element of the array at _n_ = 5,702,887, and 0.63 ns at _n_ = 9,227,465.

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

It is also presented in Martin Harriman, [Sampling Without Replacement: Durstenfeld-Fisher-Yates
Permutation for Very Large Permutation
Sizes](https://www.tdcommons.org/cgi/viewcontent.cgi?article=1485&context=dpubs_series).

My C++ and Rust implementations use a faster variant, which keeps the first _k_ entries of the
simulated array in the output array itself, so only swaps that reach beyond _k_ touch the hash
table. This variant was described by rici in a [Stack Overflow
answer](https://stackoverflow.com/a/30559195) in 2015. Thanks to Mathias Weller for bringing it to
my attention.

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
_k_ ≥ _n_/2 the book suggests switching to iterative choosing, which Nijenhuis and Wilf call
RKS2; I benchmark RANKSB on its own.

My C++ and Rust implementations follow the program in the book, except that they fill each bin
as soon as they know where its values go, rather than first marking out every bin's slots and
then scanning for the marks; the Python one keeps the counts in a separate array, which is easier
to follow.

* Albert Nijenhuis and Herbert S. Wilf
    * Combinatorial Algorithms for Computers and Calculators, 2nd edition
    * Academic Press, 1978
    * Chapter 4, "Random _k_-Subset of an _n_-Set (RANKSB)"

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

The code printed in the proceedings is wrong: it can divide by zero, and the last step has an
off-by-one error that biases the output. The authors' corrected version of the paper and the
errata in their repository fix both, and my implementations apply those corrections. Thanks to metasj for bringing this algorithm to my attention.

* Michael Shekelyan and Graham Cormode
    * Sequential Random Sampling Revisited: Hidden Shuffle Method
    * Proceedings of the 24th International Conference on Artificial Intelligence and Statistics
      (AISTATS 2021), PMLR volume 130
    * https://proceedings.mlr.press/v130/shekelyan21a.html
    * Corrected version: https://compsci.science/shekelyan/hiddenshuffle.pdf
    * Errata and the authors' implementations: https://github.com/shekelyan/sampleiterator

## Cardchoose

Algorithm | Python | C++ | Rust | Order guarantee | Data structures | Time
----|----|----|----|----|----|----
[Cardchoose](cardchoose.md) | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | [Rust](rust/src/algorithms/cardchoose.rs) | Sorted | none | _k_ log _k_

This is my algorithm: a Pólya urn generates a uniform
multiset, and stars and bars turns it into a uniform subset. [Detailed
description](cardchoose.md), including how it relates to [prior art](priorart.md).

## Algorithms not benchmarked

The literature holds many more algorithms than the thirteen I time, and my coverage is not
complete. These are the ones I know of and have not timed; for some I give a reason, and for
others there is none beyond the size of the list.

* **Ahrens and Dieter's SG, SG\* and SH**, all for sorted output. SG takes each value
  independently with probability a little over _k_/_n_, drawing the gaps between them from a
  geometric distribution in floating point; it starts again if it has fewer than _k_ values, and
  otherwise discards the excess at random. It needs an array a little longer than _k_. SG\* needs
  none: it runs the generator once to count the values, restores its state, and runs it again,
  keeping exactly _k_ by iterative choosing. SH is rejection sampling into a hash table whose hash
  is the value scaled down, so that the table is nearly in order and an insertion sort finishes
  the job. Their authors found SG and SH about equally fast, and four to five times as fast as the
  Algorithm D of the time.
* **Teuhola and Nevalainen's HTRY** is set-based rejection sampling with its hash table packed
  into the output array, each entry holding a value and a link, so that it needs no memory beyond
  the output. The packing is intricate, and in its authors' own timings (their Figure 4) it takes
  two to three times as long as their other hash-based algorithm until _k_ exceeds _n_/2.
* **Teuhola and Nevalainen's version of HSEL** needs 2_k_ storage locations rather than about
  5_k_. It draws all _k_ random numbers first and then works out which value each one stands for,
  from the last back.
* **Gupta and Bhattacharjee's GSEL**, and Rouzankin and Voytishek's algorithms that hold the
  chosen values in a sorted array or an unbalanced search tree, draw each value as a rank among
  those not yet chosen (see [prior art](priorart.md)). They need memory that Cardchoose does not,
  and GSEL follows pointers through a tree for every value drawn.
* **Rouzankin and Voytishek's halving algorithm** makes about log _k_ draws for each value (see
  [prior art](priorart.md)).
* **Other ways to draw the skips of a sequential sampler** such as Algorithm D. Devroye (Chapter
  XII §§3–4) gives several, and oversampling, which rounds sorted uniforms to integers and repairs
  the repeats afterwards. Ting observes that the skip is a beta-binomial variate, so that the loop
  takes only a few lines wherever a library provides a binomial generator; it takes O(_k_) time
  when that generator takes constant expected time, and the work of Algorithm D's rejection step
  is then done inside it. Like Algorithm D, these draw their skips in floating point.
* **Drawing with replacement, sorting and removing duplicates.** Bentley (_Programming Pearls_,
  §12.3) suggests drawing somewhat more than _k_ values, sorting them, removing the duplicates,
  and taking a sorted sample of _k_ from what is left. It needs room for the excess, and a second
  sampler to trim it.

[Prior art](priorart.md) gives the references for all of these.
