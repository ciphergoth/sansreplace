import random

def choose(n, k):
    "k distinct integers 0 <= x < n"
    d = set()
    for i in range(n - k, n):
        j = random.randrange(i+1)
        if j in d:
            d.add(i)
        else:
            d.add(j)
    return list(d)
