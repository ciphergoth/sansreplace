# To do

Open work on the paper, the benchmarks and the proposal to `rand`. Add an item here as soon as it
comes up, and delete it when it is done.

## Next

- Push the one unpushed commit in terraform-config (`df476b0`).
- Email Daniel Ting: his sparse Fisher–Yates sampler is Ernvall and Nevalainen's HSEL (1982), and
  Dahl's algorithm in Knuth's second edition (1981) is earlier still. His blog post already
  credits Bui (2015) and Dreik.

## Decisions

- Whether to time Sanders et al.'s Algorithm R and Ahrens and Dieter's SG. Both are reported
  faster than Algorithm D, and the paper lists them under "Algorithms not benchmarked". Timing
  them means implementations in Python, C++ and Rust, a benchmark run, and revised results and
  choosing sections. SG fits the calling convention as it is: it would allocate its buffer of
  ⌊_k_ + 4 _a_ √_k_⌋ entries inside the call, as the hash-based algorithms allocate their
  tables, and write the _k_ it keeps into the output array. Keep that convention: an output
  array allocated per call would bring back the alignment and page-mapping effects that
  `measurement.md` describes.
- Whether to post the paper to Hexagon as well as Zenodo. Hexagon compiles from TeX source in
  an environment it does not document; its API suggests LuaLaTeX is accepted and says nothing
  of XeLaTeX, biber or minted.

## Paper

- Read the PDF by eye for layout. The text and the references were checked after each change in
  the references pass; the layout was not, and the whole design has changed since: STIX Two,
  Libre Franklin headings, Inconsolata, spaced paragraphs and ragged right. Look at the bulleted
  subsection "Algorithms not benchmarked" and the bibliography.
- The paper's first version is published on Zenodo as `10.5281/zenodo.24545118`, built from the
  tag `paper_v1`; `10.5281/zenodo.24545117` covers all versions. A second version needs: "New
  version" on the record at zenodo.org, which gives a draft with a new number and DOI; that
  number and a new tag, `paper_v2`, in `latex/zenodo_draft` and in `latex/cardchoose.tex`, with
  the new date; a GitHub release on the tag carrying `runs.tar.xz`; then `latex/zenodo_draft`
  and Publish. The script has only been run against a first version, so check it against the
  new draft before trusting it.
- `latex/make_arxiv` and the tag name `arxiv_01` are left over from the arXiv submission.

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
- The issue is to cite the paper for the fairness proof by its Zenodo DOI,
  `10.5281/zenodo.24545117`.

## Housekeeping

- Decide whether the benchmark machine keeps a pool of 2 MB pages. The recommendation was to
  leave it as it is.
