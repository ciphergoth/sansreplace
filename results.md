# C++ results

`cardchoose` outperforms `rejectionsample`, `floydf2`, and `selby_fy` for all
values of _n_ and _k_. `quadraticreject` is good where _k_ < 100 or so, and
`fisheryates`, `iterativechoose`, and `reservoirsample`
perform well when _k_/_n_ is large.

Temporarily pulling all results from this page in preparation for some
refactoring and re-measuring.