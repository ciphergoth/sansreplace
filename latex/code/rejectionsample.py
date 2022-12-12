def random_choose(n, k):
    d = []
    r = set()
    while len(d) < k:
        x = random.randrange(n)
        if x not in r:
            d.append(x)
            r.add(x)
    return d
