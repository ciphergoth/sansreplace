def random_choose(n, k):
    d = []
    for i in range(k):
        m = n - k + i
        r = random.randrange(m + 1)
        for j in range(i):
            if d[j] == r:
                d[j] = m
                break
        d.append(r)
    return d
