# The algorithms in presentation order: quadratic, then O(n), then hash-based,
# then O(k) with no hash table, then cardchoose. Generated tables, graph
# legends (plot_anim.FAMILIES) and the web explorer use this order. The paper
# and algorithms.md instead keep three explanatory pairs together: quadratic
# rejection then set-based rejection, Floyd's F2 then quadratic F2, and SELECT
# then HSEL.
ALGORITHMS = [
    "quadraticreject",
    "quadraticf2",
    "iterativechoose",
    "reservoirsample",
    "select",
    "rejectionsample",
    "floydf2",
    "hsel",
    "ranksb",
    "vitterd",
    "hiddenshuffle",
    "cardchoose",
]
