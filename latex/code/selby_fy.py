def random_choose(n, k):
    d = [None] * k
    e = {}
    for i in range(k):
        j = random.randrange(i, n)
        d[i] = e.get(j, j)
        e[j] = e.get(i, i)
    return d
