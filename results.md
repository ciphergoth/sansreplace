# Results

These results were measured on an Apple M4 MacBook Pro with 16GB of memory, using clang 21.1 and
rustc 1.98, for values of _n_ and _k_ that are Fibonacci numbers, with _k_ at most 102,334,155
(the 40th Fibonacci number). [Full results](results/latest/)

Since both _n_ and _k_ affect the performance, we compare algorithms
using an animated graph. For each frame, we fix the value of _n_ and
graph the performance of each algorithm for various values of _k_;
we graph time/_k_ to show the cost in ns per number we output. Since
we are plotting a linear/log graph of time/_k_, the _k_ log _k_
performance of the algorithm is visible as a straight line.

## C++

The C++ implementations use the standard library's `std::unordered_set` and `std::unordered_map`,
so these results show the performance you get from a standard data structure.
`cardchoose` outperforms `rejectionsample`, `floydf2`, and `hsel` for all
values of _n_ and _k_. `quadraticreject` and `quadraticf2` are good where _k_ < 100 or so, and
`select`, `iterativechoose`, and `reservoirsample`
perform well when _k_/_n_ is large.

![graph](results/latest/cpp-random.webp)

![graph](results/latest/cpp-sorted.webp)

## Rust

The Rust port's hash-based algorithms use Rust's standard `HashMap` and `HashSet`. These are flat
open-addressing hash tables, much faster than the node-based C++ ones; the hash-based algorithms
take about 0.4–0.5× as long in Rust as in C++, while `cardchoose` takes about 0.6×.

In random order, `rejectionsample` beats `cardchoose` for _k_ between about 100 and
10<sup>6</sup> when _k_ is a small fraction of _n_. `cardchoose` is still faster around _k_ = 10,
around _k_ = 10<sup>7</sup>, where the hash table no longer fits in cache, and when _k_/_n_ is
large. `cardchoose` is never more than 1.38× slower than the fastest of `rejectionsample`,
`floydf2`, and `hsel`, while each of those is at least 13× slower than the fastest somewhere. In
sorted order, `cardchoose` outperforms `rejectionsample`, `floydf2`, and `hsel` for all
values of _n_ and _k_.

![graph](results/latest/rust-random.webp)

![graph](results/latest/rust-sorted.webp)

## Comparison

The table compares `cardchoose` with `rejectionsample`, `floydf2`, and `hsel` at the points
with _k_ ≥ 10, giving `cardchoose`'s time as a fraction of the fastest of the other three.

| | `cardchoose` fastest | median | range |
|---|---|---|---|
| C++, random order | 715 of 715 | 0.62× | 0.11–0.95× |
| C++, sorted order | 715 of 715 | 0.37× | 0.05–0.52× |
| Rust, random order | 369 of 715 | 1.00× | 0.08–1.38× |
| Rust, sorted order | 707 of 707 | 0.47× | 0.03–0.58× |

The next table gives each algorithm's worst case over the same points: the largest factor by
which it is slower than the fastest of the four.

| | `cardchoose` | `rejectionsample` | `floydf2` | `hsel` |
|---|---|---|---|---|
| C++, random order | 1.00× | 91× | 15× | 8.7× |
| C++, sorted order | 1.00× | 172× | 23× | 19× |
| Rust, random order | 1.38× | 100× | 13× | 13× |
| Rust, sorted order | 1.00× | 269× | 32× | 49× |

The worst cases for the hash-based algorithms are where _k_ = _n_. Leaving out the points where
_k_ ≥ _n_/2, they are still 2.5–6.4× slower than the fastest somewhere, mostly at large _k_, where
their hash tables no longer fit in cache.
