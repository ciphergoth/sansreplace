import random

def sorted_choose(n, k):
    "k distinct integers 0 <= x < n, sorted"
    d = []
    for i in range(n):
        if random.randrange(n - i) < k - len(d):
            d.append(i)
    return d

def random_choose(n, k):
    d = sorted_choose(n, k)
    for i in range(k):
        j = random.randrange(i+1)
        d[i], d[j] = d[j], d[i]
    return d
