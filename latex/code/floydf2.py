def set_choose(n, k):
    d = set()
    for m in range(n - k, n):
        j = random.randrange(m+1)
        if j in d:
            d.add(m)
        else:
            d.add(j)
    return d
