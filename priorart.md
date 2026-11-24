# Prior art

How [Cardchoose](cardchoose.md) relates to earlier work.

Physicists have names for the three distributions in the Cardchoose story. Distinguishable
particles placed independently in cells, the biased way of picking a multiset, is
Maxwell–Boltzmann statistics. Indistinguishable particles with every occupancy pattern equally
likely, a uniform multiset, is Bose–Einstein. At most one particle per cell with every pattern
equally likely, a uniform subset, is Fermi–Dirac. So the one-line summary of Cardchoose is:
generate a Bose–Einstein configuration with a Pólya urn, then map it to Fermi–Dirac by stars and
bars.

Both halves are long established. Cardchoose's multiset loop
is a **Pólya urn**: start with _t_ balls, one of each colour; draw a ball uniformly at random, and
put it back along with a new ball of the same colour. `randrange(t + i)` picks one of the _t_+_i_
balls in the urn, and `d[r - t]` is what drawing a copy means. The urn is a textbook model, and that
with one ball of each colour its counts are uniform, which is to say Bose–Einstein, is a standard
lemma about it (Levin and Peres, Lemma 2.7); Niven and Grendar map all three statistics onto it.
In Bayesian terms, each draw is Laplace's rule of succession (Feller,
chapter V §2) for _t_ outcomes, and uniform counts are what the scholium of Bayes's essay takes to
characterise knowing nothing beforehand.

I have not found the urn set out as an algorithm, though. Its draws can be generated from the other
side of an equivalence that Blackwell and MacQueen prove for a more general urn: draw a vector of
probabilities from a Dirichlet distribution, then sample from it independently. That takes
floating-point arithmetic and memory proportional to _t_. The device in Cardchoose's first loop,
drawing by copying a uniformly random earlier entry of an array, does appear in Batagelj and
Brandes's generator for preferential-attachment graphs, which runs a related urn where each step
also adds a new colour. The combinatorial generation literature does generate random multisets, but
through stars and bars in the other direction: Nijenhuis and Wilf's RANCOM chooses a random
(_k_-1)-subset of {1, …, _n_+_k_-1} and differences it. Nor do surveys of sampling without
replacement describe anything like Cardchoose: Devroye's has no urn and no stars and bars, and
Ting's reworkings of the established algorithms are a sparse Fisher–Yates sampler, which is Ernvall
and Nevalainen's HSEL, the swapping method on an existing array with its swaps undone afterwards,
and a sorted sampler that draws each skip from a beta-binomial distribution in floating point. The
nearest I have found to drawing first and sorting afterwards is a sampler that Bentley sketches
(_Programming Pearls_, §12.3), which draws with replacement, sorts, and removes duplicates.

Five other algorithms need no memory beyond the output array and take O(_k_) expected time:
Teuhola and Nevalainen's HTRY, Nijenhuis and Wilf's RANKSB, Ahrens and Dieter's SG\*, Vitter's
Algorithm D and Shekelyan and Cormode's Hidden Shuffle. Hidden Shuffle postdates Cardchoose, and
the last three need floating-point arithmetic. I have not benchmarked HTRY or SG\*; [algorithms
not benchmarked](algorithms.md#algorithms-not-benchmarked) describes them.

Gupta and Bhattacharjee's GSEL also takes O(_k_ log _k_) time and can return sorted output. Like
SELECT, it draws each value as a uniform rank among the values not yet chosen, and it finds the
value with that rank in a balanced tree of the values chosen so far, as Ernvall and Nevalainen had
suggested; Rouzankin and Voytishek use a sorted array or an unbalanced search tree instead. The
tree needs memory that Cardchoose does not, and following pointers through it for every value
drawn is unlikely to beat one sort over a flat array. Rouzankin and Voytishek's halving algorithm
also takes O(_k_ log _k_) expected time and returns sorted output, with only a recursion stack
beside the output array: it splits the range in two, decides with one draw for each value which
half it falls in, and recurses, so it makes about log _k_ draws for each value where Cardchoose
makes one.

* William Feller, _An Introduction to Probability Theory and Its Applications_, volume 1, Wiley.
  Chapter II §5 covers occupancy problems and the three statistics; chapter V §2 covers urn
  models.
* David A. Levin and Yuval Peres, _Markov Chains and Mixing Times_, 2nd edition, American
  Mathematical Society, 2017. §2.4, Lemma 2.7.
* Robert K. Niven and Marian Grendar, "Generalized classical, quantum and intermediate statistics
  and the Pólya urn model", _Physics Letters A_ 373(6), 621–626, 2009.
  [arXiv:0808.2102](https://arxiv.org/abs/0808.2102).
* Thomas Bayes and Richard Price, "An essay towards solving a problem in the doctrine of chances",
  _Philosophical Transactions of the Royal Society of London_ 53, 370–418, 1763.
  [doi:10.1098/rstl.1763.0053](https://doi.org/10.1098/rstl.1763.0053).
* David Blackwell and James B. MacQueen, "Ferguson distributions via Pólya urn schemes",
  _Annals of Statistics_ 1(2), 353–355, 1973.
  [doi:10.1214/aos/1176342372](https://doi.org/10.1214/aos/1176342372).
* Vladimir Batagelj and Ulrik Brandes, "Efficient generation of large random networks", _Physical
  Review E_ 71, 036113, 2005.
  [doi:10.1103/PhysRevE.71.036113](https://doi.org/10.1103/PhysRevE.71.036113).
* P. Gupta and G. P. Bhattacharjee, "An efficient algorithm for random sampling without
  replacement", _Foundations of Software Technology and Theoretical Computer Science_, Lecture
  Notes in Computer Science 181, 435–442, Springer, 1984.
  [doi:10.1007/3-540-13883-8_89](https://doi.org/10.1007/3-540-13883-8_89).
* P. S. Rouzankin and A. V. Voytishek, "On the cost of algorithms for random selection", _Monte
  Carlo Methods and Applications_ 5(1), 39–54, 1999.
  [doi:10.1515/mcma.1999.5.1.39](https://doi.org/10.1515/mcma.1999.5.1.39).
* Jarmo Ernvall and Olli Nevalainen, "An algorithm for unbiased random sampling", _The Computer
  Journal_ 25(1), 45–47, 1982. [doi:10.1093/comjnl/25.1.45](https://doi.org/10.1093/comjnl/25.1.45).
* J. Teuhola and O. Nevalainen, "Two efficient algorithms for random sampling without
  replacement", _International Journal of Computer Mathematics_ 11(2), 127–140, 1982.
  [doi:10.1080/00207168208803304](https://doi.org/10.1080/00207168208803304).
* J. H. Ahrens and U. Dieter, "Sequential random sampling", _ACM Transactions on Mathematical
  Software_ 11(2), 157–169, 1985.
  [doi:10.1145/214392.214402](https://doi.org/10.1145/214392.214402).
* Jon Bentley, _Programming Pearls_, 2nd edition, Addison-Wesley, 2000. Columns 1, 12 and 13.
* Luc Devroye, _Non-Uniform Random Variate Generation_, Springer-Verlag, 1986. Chapter XII,
  "Random sampling". [Online](https://luc.devroye.org/rnbookindex.html).
* Daniel Ting, "Simple, optimal algorithms for random sampling without replacement", 2021.
  [arXiv:2104.05091](https://arxiv.org/abs/2104.05091).
* Albert Nijenhuis and Herbert S. Wilf, _Combinatorial Algorithms for Computers and Calculators_,
  2nd edition, Academic Press, 1978. Chapter 4, "Random _k_-Subset of an _n_-Set (RANKSB)", and
  chapter 6, "Random Composition of n into k Parts (RANCOM)".
* Jeffrey Scott Vitter, "An efficient algorithm for sequential random sampling", _ACM Transactions
  on Mathematical Software_ 13(1), 58–67, 1987.
  [doi:10.1145/23002.23003](https://doi.org/10.1145/23002.23003).
* Michael Shekelyan and Graham Cormode, "Sequential random sampling revisited: hidden shuffle
  method", _Proceedings of the 24th International Conference on Artificial Intelligence and
  Statistics (AISTATS)_, PMLR 130, 2021.
  [proceedings.mlr.press/v130/shekelyan21a](https://proceedings.mlr.press/v130/shekelyan21a.html);
  [corrected version](https://compsci.science/shekelyan/hiddenshuffle.pdf).
