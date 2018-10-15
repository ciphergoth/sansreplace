import random

def choose(n, k):
    "k distinct integers 0 <= x < n, random"
    d = []
    r = set()
    while len(d) < k:
        j = random.randrange(n)
        if j not in r:
            d.append(j)
            r.add(j)
    return d