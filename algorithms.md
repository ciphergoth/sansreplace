# Algorithms for sampling without replacement

In the standard form of the problem, the answers can be in any order and
so the order must also be chosen fairly; there is a variation in which the answers must be in sorted
order.  Any algorithm can be turned into one with a "sorted" order guarantee
with an O(_k_ log _k_) sort, or a "random" order guarantee with an O(_k_)
Fisher-Yates shuffle.

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | Random | none | _k_^2
Set-based rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | Random | Set | _k_
Iterative choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | Sorted | none | _n_
Reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | Random | none | _n_
SELECT | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | Random | n-sized list | _n_
HSEL | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | Random | Dictionary | _k_
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | none | Set | _k_
Quadratic F2 | [Python](python/algorithms/quadraticf2.py) | [C++](cpp/quadraticf2.cpp) | Random | none | _k_^2
[Cardchoose](cardchoose.md) | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | Sorted | none | _k_ log _k_
Vitter's Algorithm D | [Python](python/algorithms/vitterd.py) | [C++](cpp/vitterd.cpp) | Sorted | none | _k_

## Quadratic rejection sampling

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | Random | none | _k_^2

For each output, try choosing
an integer in [0, _n_), but check every integer already in the output to see if
it's the same, and reject and re-draw if so. This is quadratic in _k_, but for small
_k_ the small constants can mean this algorithm does very well.

## Set-based rejection sampling

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Set-based rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | Random | Set | _k_

Maintain a set of already-produced values. While the output isn't full,
try choosing a random integer in [0, _n_). If this is in the set of already-produced
values, discard it. Otherwise, append it to the output, and add it to the list of
already-produced values.

One of the simplest possible algorithms, but the cost of set lookups and insertions
is high.

## Iterative choosing (Algorithm S)

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Iterative choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | Sorted | none | _n_

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

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | Random | none | _n_

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

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
SELECT | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | Random | n-sized list | _n_

Initialize an array of the values [0, _n_). Pick out values at random to include
in the output, and delete them by replacing them with a value from one end and
shortening the array.

This is known as SELECT in Ernvall and Nevalainen, though in a slight variation
we pick off the array at the start, rather than the end.

## HSEL

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
HSEL | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | Random | Dictionary | _k_

This is a variation of the above in which we use a hash table to simulate the array
of values [0, _n_), and store only where we change it. 
First proposed by Ernvall and Nevalainen in 1982:

* Jarmo Ernvall and Olli Nevalainen
    * An algorithm for unbiased random sampling
    * The Computer Journal Volume 25, Number 1, February, 1982
    * 45--47
    * https://academic.oup.com/comjnl/article/25/1/45/527401

It is also presented in Martin Harriman, [Sampling Without Replacement: Durstenfeld-Fisher-Yates
Permutation for Very Large Permutation
Sizes](https://www.tdcommons.org/cgi/viewcontent.cgi?article=1485&context=dpubs_series).

## Floyd's F2

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | none | Set | _k_

I refer you to two excellent explanations:

* [A Sample of Brilliance](https://blog.acolyer.org/2018/01/30/a-sample-of-brilliance/)
* [Robert Floyd's Tiny and Beautiful Algorithm for Sampling without Replacement](http://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html)

The original presentation is as one of Jon Bentley's Programming Pearls, though the algorithm is due to Robert W Floyd: [A Sample of Brilliance](https://doi.org/10.1145/30401.315746).

## Quadratic F2

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Quadratic F2 | [Python](python/algorithms/quadraticf2.py) | [C++](cpp/quadraticf2.cpp) | Random | none | _k_^2

A quadratic variant on Floyd's F2 which needs no external data structure. Inspired by the
implementation in the [Rust rand crate](https://github.com/rust-random/rand/blob/19897e53c1908c5a193e2aacff59170e3e72b8de/src/seq/index.rs#L231),
but with some optimizations. Like quadratic rejection sampling, the small constants mean
this does very well for small _k_.

## Cardchoose

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
[Cardchoose](cardchoose.md) | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | Sorted | none | _k_ log _k_

This is my algorithm, first made public in October 2018: a Pólya urn generates a uniform
multiset, and stars and bars turns it into a uniform subset. [Detailed
description](cardchoose.md), including how it relates to [prior art](priorart.md).

## Vitter's Algorithm D

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Vitter's Algorithm D | [Python](python/algorithms/vitterd.py) | [C++](cpp/vitterd.cpp) | Sorted | none | _k_

A sequential sampling algorithm: like iterative choosing, it produces the output in sorted order, but
instead of deciding about each value in [0, _n_) in turn, it draws the number of values to skip
before the next chosen one directly. The skip has a known discrete distribution, and Algorithm D
samples it by rejection from a continuous approximation, which takes O(1) expected time per
output, so the whole algorithm takes O(_k_) expected time and needs no extra memory. The cost is
floating-point arithmetic: each output needs a few uniform variates and exponentiations. Once
_n_ is at most 13 _k_, it falls back on Vitter's Algorithm A, which draws each skip in O(skip) time,
so the total work for the rest of the output is O(_n_).

Our implementation follows the Pascal-like code in the paper's appendix.

* Jeffrey Scott Vitter
    * An Efficient Algorithm for Sequential Random Sampling
    * ACM Transactions on Mathematical Software, Volume 13, Number 1, March 1987
    * 58--67
    * https://doi.org/10.1145/23002.23003
