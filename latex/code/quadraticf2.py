def set_choose(n, k):
    d = []
    for m in range(n - k, n):
        j = random.randrange(m + 1)
        if j in d:
            d.append(m)
        else:
            d.append(j)
    return d
