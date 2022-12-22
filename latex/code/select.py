def random_choose(n, k):
    d = [None] * k
    e = list(range(n))
    for i in range(k):
        j = random.randrange(i, n)
        d[i] = e[j]
        e[j] = e[i]
    return d