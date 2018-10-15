import random

def choose(n, k):
    "k distinct integers 0 <= x < n, random"
    d = []
    e = {}
    for i in range(k):
        j = random.randrange(i, n)
        d.append(e.get(j, j))
        e[j] = e.get(i, i)
    return d
