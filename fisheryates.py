import random

def choose(n, k):
    "k distinct integers 0 <= x < n, random"
    d = [None] * k
    e = list(range(n))
    for i in range(k):
        j = random.randrange(i, n)
        d[i] = e[j]
        e[j] = e[i]
    return d
