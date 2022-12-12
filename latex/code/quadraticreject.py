def random_choose(n, k):
    d = []
    while len(d) < k:
        x = random.randrange(n)
        if x not in d:
            d.append(x)
    return d
