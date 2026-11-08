# Prior art

How [Cardchoose](cardchoose.md) relates to earlier work.

Physicists have names for the three distributions in the Cardchoose story. Distinguishable
particles placed independently in cells, the biased way of picking a multiset, is
Maxwell–Boltzmann statistics. Indistinguishable particles with every occupancy pattern equally
likely, a uniform multiset, is Bose–Einstein. At most one particle per cell with every pattern
equally likely, a uniform subset, is Fermi–Dirac. Stars and bars is the standard way to count the
Bose–Einstein configurations, by matching them with Fermi–Dirac ones. So the one-line summary of
Cardchoose is: generate a Bose–Einstein configuration with a Pólya urn, then map it to
Fermi–Dirac by stars and bars.

The mathematics of both halves is classical. Cardchoose's multiset loop is a **Pólya urn**: start
with _t_ balls, one of each colour; draw a ball uniformly at random, and put it back along with a
new ball of the same colour. `randrange(t + i)` picks one of the _t_+_i_ balls in the urn, and
`d[r - t]` is what drawing a copy means. The urn is a textbook object of study, and that with one
ball of each colour its counts are uniform, which is to say Bose–Einstein, is a standard lemma
about it (Levin and Peres, Lemma 2.7); Niven and Grendar map all three statistics onto the urn,
with Maxwell–Boltzmann as replacement without a copy, Bose–Einstein as replacement with a copy,
and Fermi–Dirac as no replacement. But the urn is presented as a model, not as a way to generate
anything, beyond passing remarks that it could be used so, as in Cancrini and Posta. The nearest
thing to running an urn as a sampler in earnest is in Bayesian statistics, where the
Blackwell–MacQueen urn draws an exchangeable sequence from a Dirichlet process, which is a
different purpose. The combinatorial generation literature does generate random multisets, but
through stars and bars in the other direction: Nijenhuis and Wilf's RANCOM chooses a random
(_k_-1)-subset of {1, …, _n_+_k_-1} and differences it. Nor do surveys of sampling without
replacement describe anything like it: Devroye's has no urn and no stars and bars, and Ting,
reworking the classical algorithms into simpler forms, gives a sparse Fisher–Yates sampler,
essentially HSEL, and a sorted sampler that draws each gap from a beta-binomial distribution,
which needs floating point. What I have not found elsewhere is the
urn presented as a generator for uniform multisets, or its composition with stars and bars into
a sampler for sampling without replacement. Needing no set, map or extra memory is not new in
itself: Nijenhuis and Wilf's RANKSB also uses nothing but the output array, and so do Vitter's
Algorithm D and, more recently, Shekelyan and Cormode's Hidden Shuffle, though they need
floating-point arithmetic where Cardchoose and RANKSB use only exact integer draws.

The earlier algorithm closest to Cardchoose is Gupta and Bhattacharjee's GSEL, which also takes
O(_k_ log _k_) time and can return sorted output. Like SELECT, it draws its _i_th value uniformly
from the _n_ − _i_ + 1 values not yet chosen, as a rank _r_ among them; that value is _r_ + _u_,
where _u_ is the number of values already chosen below it, and GSEL finds _u_ in a balanced
binary tree of the chosen values that records the size of each subtree. Adding to each value the
number of chosen values below it is also the last step of Cardchoose, where each sorted value has
its index added. But GSEL must keep its chosen values in order to count as it goes, and so needs a
tree of _k_ nodes, while the urn never needs the earlier values in order, so Cardchoose can leave
the counting to a single sort at the end, in the output array. I would be glad to hear of any
earlier description of Cardchoose.

Sorted output without a sort has a classic answer for real numbers: Bentley and Saxe generate _k_
sorted uniform reals in O(_k_) time, drawing the largest as _U_^(1/_k_) and repeating for the
rest below it, and credit Friedman with an essentially equivalent earlier program. This can't
stand in for the urn: sorted independent draws, even rounded to integers, follow the
Maxwell–Boltzmann distribution over multisets, not the Bose–Einstein one. Its closest analogue
for distinct integers is Vitter's Algorithm D, whose continuous approximation is the same step.

* William Feller, _An Introduction to Probability Theory and Its Applications_, volume 1, Wiley.
  Chapter II §5 covers occupancy problems and the three statistics; chapter V §2 covers urn
  models.
* David A. Levin and Yuval Peres, _Markov Chains and Mixing Times_, 2nd edition, American
  Mathematical Society, 2017. §2.4, Lemma 2.7.
* Robert K. Niven and Marian Grendar, "Generalized classical, quantum and intermediate statistics
  and the Pólya urn model", _Physics Letters A_ 373(6), 621–626, 2009.
  [arXiv:0808.2102](https://arxiv.org/abs/0808.2102).
* Nicoletta Cancrini and Gustavo Posta, "Propagation of chaos for a general balls into bins
  dynamics", [arXiv:1906.03876](https://arxiv.org/abs/1906.03876), 2019.
* David Blackwell and James B. MacQueen, "Ferguson distributions via Pólya urn schemes",
  _Annals of Statistics_ 1(2), 353–355, 1973.
* P. Gupta and G. P. Bhattacharjee, "An efficient algorithm for random sampling without
  replacement", _Foundations of Software Technology and Theoretical Computer Science_, Lecture
  Notes in Computer Science 181, 435–442, Springer, 1984.
  [doi:10.1007/3-540-13883-8_89](https://doi.org/10.1007/3-540-13883-8_89).
* Luc Devroye, _Non-Uniform Random Variate Generation_, Springer-Verlag, 1986. Chapter XII,
  "Random sampling". [Online](https://luc.devroye.org/rnbookindex.html).
* Daniel Ting, "Simple, optimal algorithms for random sampling without replacement", 2021.
  [arXiv:2104.05091](https://arxiv.org/abs/2104.05091).
* Albert Nijenhuis and Herbert S. Wilf, _Combinatorial Algorithms for Computers and Calculators_,
  2nd edition, Academic Press, 1978. Chapter 4, "Random _k_-Subset of an _n_-Set (RANKSB)", and
  chapter 6, "Random Composition of n into k Parts (RANCOM)".
* Jon Louis Bentley and James B. Saxe, "Generating sorted lists of random numbers", _ACM
  Transactions on Mathematical Software_ 6(3), 359–364, 1980.
  [doi:10.1145/355900.355907](https://doi.org/10.1145/355900.355907).
* Jerome H. Friedman, "Random event generation with preferred frequency distributions", _Journal
  of Computational Physics_ 7(2), 201–218, 1971.
* Jeffrey Scott Vitter, "An efficient algorithm for sequential random sampling", _ACM Transactions
  on Mathematical Software_ 13(1), 58–67, 1987.
  [doi:10.1145/23002.23003](https://doi.org/10.1145/23002.23003).
* Michael Shekelyan and Graham Cormode, "Sequential random sampling revisited: hidden shuffle
  method", _Proceedings of the 24th International Conference on Artificial Intelligence and
  Statistics (AISTATS)_, PMLR 130, 2021.
  [proceedings.mlr.press/v130/shekelyan21a](https://proceedings.mlr.press/v130/shekelyan21a.html).
