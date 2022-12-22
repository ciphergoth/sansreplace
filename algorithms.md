# Algorithms for sampling without replacement

In the standard form of the problem, the answers can be in any order and
so the order must also be chosen fairly; there is a variation in which the answers must be in sorted
order.  Any algorithm can be turned into one with a "sorted" order guarantee
with an O(_k_ log _k_) sort, or a "random" order guarantee with an O(_k_)
Fisher-Yates shuffle.

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | Random | Set | _k_
quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | Random | none | _k_^2
iterative random choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | Sorted | none | _n_
reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | Random | none | _n_
Python-style Fisher-Yates | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | Random | n-sized list | _n_
HSAMPLE | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | Random | Dictionary | _k_
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | none | Set | _k_
"[cardchoose](cardchoose.md)" | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | Sorted | none | _k_ log _k_

## Rejection sampling

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | Random | Set | _k_

Maintain a set of already-produced values. While the output isn't full,
try choosing a random integer in [0, _n_). If this is in the set of already-produced
values, discard it. Otherwise, append it to the output, and add it to the list of
already-produced values.

One of the simplest possible algorithms, but the cost of set lookups and insertions
is high.

## Quadratic rejection sampling

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | Random | none | _k_^2

Even simpler than rejection sampling. For each output, try choosing
an integer in [0, _n_), but check every integer already in the output to see if
it's the same, and reject and re-draw if so. This is quadratic in _k_, but for small
_k_ the small constants can mean this algorithm does very well.

## Iterative random choosing

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
iterative random choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | Sorted | none | _n_

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

## Reservoir sampling

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | Random | none | _n_

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

## Python-style Fisher-Yates

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
Python-style Fisher-Yates | [Python](python/algorithms/select.py) | [C++](cpp/select.cpp) | Random | n-sized list | _n_

Initialize an array of the values [0, _n_). Pick out values at random to include
in the output, and delete them by replacing them with a value from one end and
shortening the array.

This is known as SAMPLE in Ernvall and Nevalainen, though in a slight variation
we pick off the array at the start, rather than the end.

## Ernvall-Nevalainen HSAMPLE

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
HSAMPLE | [Python](python/algorithms/hsel.py) | [C++](cpp/hsel.cpp) | Random | Dictionary | _k_

This is a variation of the above in which we use a hash table to simulate the array
of values [0, _n_), and store only where we change it. I called this hsel in my
code because my friend Alex Selby first suggested it to me, but it turns out to have been
first proposed by Ernvall and Nevalainen in 1982:

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

## Cardchoose

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
"[cardchoose](cardchoose.md)" | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | Sorted | none | _k_ log _k_

This is my new algorithm. [Detailed description](cardchoose.md).
