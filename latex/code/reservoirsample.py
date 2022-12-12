def random_choose(n, k):
    d = [None] * k
    for i in range(k):
        r = random.randrange(i + 1)
        d[i] = d[r]
        d[r] = i
    for i in range(k, n):
        r = random.randrange(i + 1)
        if r < k:
            d[r] = i
    return d
