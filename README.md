# Sampling without replacement

The problem of __sampling without replacement__: given integers _n_, _k_ such that 0 ≤ _k_ ≤ _n_,
return _k_ **distinct** nonnegative integers < _n_, choosing at random such that every possible
result is equally likely.  In the standard form of the problem, the answers can be in any order and
so the order must also be chosen fairly; there is a variation in which the answers must be in sorted
order. One way to get a precise understanding of the problem is to look at the simplest algorithm
for solving it, via these [Python](python/algorithms/quadraticreject.py) or
[C++](cpp/quadraticreject.cpp) implementations.

Here, I propose an algorithm I call "[cardchoose](cardchoose.md)" for this
problem which as far as I know is novel, and which in C++ outperforms all other
solutions to either problem for most (_n_, _k_) inputs. The only data structure
used is the output vector, so no extra memory is needed; it runs in O(_k_ log
_k_) time. I've implemented this and
[seven other algorithms](algorithms.md) in C++ and Python
to compare their performance.

 Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp) | Random | Set | _k_
quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp) | Random | none | _k_^2
iterative random choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp) | Sorted | none | _n_
reservoir sampling | [Python](python/algorithms/reservoirsample.py) | [C++](cpp/reservoirsample.cpp) | Random | none | _n_
Python-style Fisher-Yates | [Python](python/algorithms/fisheryates.py) | [C++](cpp/fisheryates.cpp) | Random | n-sized list | _n_
HSAMPLE | [Python](python/algorithms/selby_fy.py) | [C++](cpp/selby_fy.cpp) | Random | Dictionary | _k_
Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp) | none | Set | _k_
"[cardchoose](cardchoose.md)" | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | Sorted | none | _k_ log _k_

In [my C++ tests](results.md), `cardchoose` outperforms `rejectionsample`,
`floydf2`, and `selby_fy` for all values of _n_ and _k_. `quadraticreject` is
best where _k_ < 100 or so, and `fisheryates` and `iterativechoose` perform well
when _k_/_n_ is large.  `cardchoose` always performs within an acceptable factor of other algorithms.

Any algorithm can be turned into one with a "sorted" order guarantee
with an O(_k_ log _k_) sort, or a "random" order guarantee with an O(_k_)
Fisher-Yates shuffle.

This is not an officially supported Google product.
