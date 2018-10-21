# Selection without replacement

[This algorithm](cardchoose.md) selects _k_ distinct integers 0 <= _x_ < _n_ at
random and returns them in sorted order. It takes O(_k_ log _k_) time and needs
no auxiliary data structures, just the list it will return things in. I've
implemented it in Python and C++, and compared it to some other options.

In [my C++ tests](results.md), `cardchoose` outperforms `rejectionsample`,
`floydf2`, and `selby_fy` for all values of _n_ and _k_. `quadraticreject` is
best where _k_ < 100 or so, and `fisheryates` and `iterativechoose` perform well
when _k_/_n_ is large.

Order guarantee | Time | Data structures | Algorithm | Python | C++
----|----|----|----|----|----
Sorted | _k_ log _k_ | none | this algorithm | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp)
Random | _k_  | Set | rejection sampling | [Python](python/algorithms/rejectionsample.py) | [C++](cpp/rejectionsample.cpp)
Random | _k_^2  | none | quadratic rejection sampling | [Python](python/algorithms/quadraticreject.py) | [C++](cpp/quadraticreject.cpp)
Sorted | _n_ | none | iterative random choosing | [Python](python/algorithms/iterativechoose.py) | [C++](cpp/iterativechoose.cpp)
Random | _n_ | n-sized list | Python-style Fisher-Yates | [Python](python/algorithms/fisheryates.py) | [C++](cpp/fisheryates.cpp)
Random | _k_ | Dictionary | Selby Fisher-Yates | [Python](python/algorithms/algorithms/selby_fy.py) | [C++](cpp/selby_fy.cpp)
none | _k_ | Set | Floyd's F2 | [Python](python/algorithms/floydf2.py) | [C++](cpp/floydf2.cpp)

Any algorithm can be turned into one with a "sorted" order guarantee
with an O(k log k) sort, or a "random" order guarantee with an O(k)
Fisher-Yates shuffle.

This is not an officially supported Google product.
