import random

def choose(n, k):
    "k distinct integers 0 <= x < n, random"
    d = [None] * k
    e = {}
    for i in range(k):
        j = random.randrange(i, n)
        d[i] = e.get(j, j)
        e[j] = e.get(i, i)
    return d
