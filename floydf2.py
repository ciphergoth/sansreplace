import random

def set_choose(n, k):
    "k distinct integers 0 <= x < n"
    d = set()
    for i in range(n - k, n):
        j = random.randrange(i+1)
        if j in d:
            d.add(i)
        else:
            d.add(j)
    return d

def random_choose(n, k):
    d = set_choose(n, k)
    rd = [None] * k
    for i, v in enumerate(d):
        j = random.randrange(i+1)
        rd[i] = rd[j]
        rd[j] = v
    return rd

def sorted_choose(n, k):
    d = list(set_choose(n, k))
    d.sort()
    return d
