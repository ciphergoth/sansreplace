# Sampling without replacement

The problem of __sampling without replacement__: given integers _n_, _k_ such that 0 ≤ _k_ ≤ _n_,
return _k_ **distinct** nonnegative integers < _n_, choosing at random such that every possible
result is equally likely. One way to get a precise understanding of the problem is to look at the
simplest algorithm for solving it, via these [Python](python/algorithms/quadraticreject.py) or
[C++](cpp/quadraticreject.cpp) implementations.

Here, I describe an algorithm I call "Cardchoose" for this problem. The only data structure used
is the output vector, so no extra memory is needed; it runs in O(_k_ log _k_) time. I've
implemented this and other algorithms in C++, Rust and Python to compare their performance.

I first proposed Cardchoose in October 2018. Its two ingredients, a Pólya urn and the
stars-and-bars bijection, are classical mathematics; what I have not found elsewhere is the urn
run as a generator, or the two composed into a sampler (see [prior art](priorart.md)). I would
be glad to hear of any earlier description.

In [my C++ tests](results.md), which use the standard library's `std::unordered_set` and
`std::unordered_map`, `cardchoose` outperforms `rejectionsample`, `floydf2`, and `hsel` for all
values of _n_ and _k_. `quadraticreject` is best where _k_ < 100 or so, and `select` and
`iterativechoose` perform well when _k_/_n_ is large. `cardchoose` always performs within an
acceptable factor of other algorithms.

A fast flat hash table changes the picture for random order output. Rust's standard `HashSet` is
one, and in Rust `rejectionsample` is up to 1.4× faster than `cardchoose` for _k_ between about
100 and 10<sup>6</sup> when _k_ is a small fraction of _n_. But `cardchoose` is never more than
about 1.4× slower than the fastest of `rejectionsample`, `floydf2`, and `hsel`, while each of those
has a bad region: at large _k_, once its hash table no longer fits in cache, it is 2.5–4× slower
than the fastest, and when _k_ is close to _n_ it is 13–100× slower. So `cardchoose` is the safe
choice for random output, and the best choice when you want sorted output, where it is still
about twice as fast as the hash-based algorithms, or when you don't have a fast hash table to hand.

* [Guide to algorithms tested](algorithms.md)
* [Description of Cardchoose](cardchoose.md)
* [Prior art](priorart.md)
* [Benchmark results](results.md)
* [Running this software](running.md)

```python
def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    t = n - k + 1
    d = [None] * k
    for i in range(k):
        r = random.randrange(t + i)
        if r < t:
            d[i] = r
        else:
            d[i] = d[r - t]
    d.sort()
    for i in range(k):
        d[i] += i
    return d
```

![graph](results/latest/cpp-sorted.webp)


This is not an officially supported Google product.
