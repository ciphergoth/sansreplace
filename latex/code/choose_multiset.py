def choose_multiset(n, k):
    m = []
    for i in range(k):
        r = random.randrange(n + i)
        if r < n:
            m.append(r)
        else:
            m.append(m[r - n])
    return m
