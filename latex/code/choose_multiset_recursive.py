def choose_multiset(n, k):
    if k == 0:
        return []
    m = choose_multiset(n, k - 1)
    r = random.randrange(n + len(m))
    if r < n:
        x = r
    else:
        x = m[r - n]
    m.append(x)
    return m
