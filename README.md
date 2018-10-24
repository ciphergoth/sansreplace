# Sampling without replacement

The problem of __sampling without replacement__: given integers _n_, _k_ such that 0 ≤ _k_ ≤ _n_,
return _k_ **distinct** nonnegative integers < _n_, choosing at random such that every possible
result is equally likely.  
One way to get a precise understanding of the problem is to look at the simplest algorithm
for solving it, via these [Python](python/algorithms/quadraticreject.py) or
[C++](cpp/quadraticreject.cpp) implementations.

Here, I propose an algorithm I call "Cardchoose" for this
problem which as far as I know is novel, and which in C++ outperforms all other
solutions to either problem for most (_n_, _k_) inputs. The only data structure
used is the output vector, so no extra memory is needed; it runs in O(_k_ log
_k_) time. I've implemented this and other algorithms in C++ and Python
to compare their performance.

In [my C++ tests](results.md), `cardchoose` outperforms `rejectionsample`,
`floydf2`, and `selby_fy` for all values of _n_ and _k_. `quadraticreject` is
best where _k_ < 100 or so, and `fisheryates` and `iterativechoose` perform well
when _k_/_n_ is large.  `cardchoose` always performs within an acceptable factor of other algorithms.

* [Guide to algorithms tested](algorithms.md)
* [Description of Cardchoose](cardchoose.md)
* [C++ benchmark results](results.md)
* [Running this software](running.md)

This is not an officially supported Google product.
