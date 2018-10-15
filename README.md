# Selection without replacement

[This algorithm](cardchoose.md) selects k integers 0 <= x < n at random
and returns them in sorted order. It takes O(k log k) time and needs no
auxiliary data structures, just the list it will return things in. I've
implemented it in Python, and compared it to some other Python options.

Any algorithm can be turned into one with a "sorted" order guarantee
with an O(k log k) sort, or a "random" order guarantee with an O(k)
Fisher-Yates shuffle.

Order guarantee | Time | Data structures | Algorithm | File
----|----|----|----|----
Sorted | k log k | none | this algorithm | `cardchoose.py`
Random | k  | Set | rejection sampling | `rejectionsample.py`
Sorted | n | none | iterative random choosing | `iterativechoose.py`
Random | n | n-sized list | Python-style Fisher-Yates | `fisheryates.py`
Random | k | Dictionary | Selby Fisher-Yates | `selby_fy.py`
none | k | Set | Floyd's F2 | `floydf2.py`

This is not an officially supported Google product.
