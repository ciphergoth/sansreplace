import random

def choose(n, k):
    "k distinct integers 0 <= x < n, random"
    d = set()
    for i in range(n - k, n):
        j = random.randrange(i+1)
        if j in d:
            d.add(i)
        else:
            d.add(j)
    d = list(d)
    for i in range(k):
        j = random.randrange(i+1)
        d[i], d[j] = d[j], d[i]
    return d
