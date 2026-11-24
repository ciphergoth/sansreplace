# To do

Open work on the paper, the benchmarks and the proposal to `rand`. Add an item here as soon as it
comes up, and delete it when it is done.

## Next

- Push sansreplace, and the one unpushed commit in terraform-config (`df476b0`).
- Email Daniel Ting: his sparse Fisher–Yates sampler is Ernvall and Nevalainen's HSEL (1982), and
  Dahl's algorithm in Knuth's second edition (1981) is earlier still. His blog post already
  credits Bui (2015) and Dreik.

## Decisions

- Whether to time Sanders et al.'s Algorithm R and Ahrens and Dieter's SG. Both are reported
  faster than Algorithm D, and the paper lists them under "Algorithms not benchmarked". Timing
  them means implementations in Python, C++ and Rust, a benchmark run, and revised results and
  choosing sections.
- Where to host the paper, since arXiv declined it.

## Paper

- Go back to a modern body font with Latin Modern for the mathematics, keeping the URL breaking,
  `emergencystretch` and `microtype` from the arXiv version.
- Read the PDF by eye for layout. The text and the references were checked after each change in
  the references pass; the layout was not. Look at the bulleted subsection "Algorithms not
  benchmarked" and the bibliography.
- A new tag and release once a host is chosen: the repository links in `latex/cardchoose.tex`
  point at the tag `arxiv_01`, and the paper has changed a great deal since. `latex/make_arxiv`
  and the tag's name are left over from the arXiv submission.

## Documents

- `algorithms.md` cites Feller for the coupon collector's problem with no entry for the book in a
  reference list.
- The Floyd's F2 entry in `algorithms.md` opens with two blog posts and only then names Bentley
  and Floyd's article.

## rand

- Open the pull request for `sample_floyd`'s scan: branch `floyd-scan` in `~/g/crates/rand`,
  draft in `scratch/rand/rand-pr-floyd-scan.md`. Its changelog line needs the pull request's
  number. Describe rand's method before 2023 as Floyd's Algorithm P in mirror image, as the paper
  now does.
- Then the issue proposing Cardchoose in place of the `BTreeSet`. Rewrite
  `scratch/rand/rand-proposal.md` first; it predates the plan to open with a pull request.
- The issue was to cite the paper for the fairness proof once it had an arXiv identifier. It needs
  the paper's new home instead.

## Housekeeping

- Decide whether the benchmark machine keeps a pool of 2 MB pages. The recommendation was to
  leave it as it is.
