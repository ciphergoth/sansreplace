import random

def choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    d = []
    for i in range(n):
        if random.randrange(n - i) < k - len(d):
            d.append(i)
    return d
