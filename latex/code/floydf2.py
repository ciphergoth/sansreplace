def set_choose(n, k):
    d = set()
    for i in range(n - k, n):
        j = random.randrange(i+1)
        if j in d:
            d.add(i)
        else:
            d.add(j)
    return d
