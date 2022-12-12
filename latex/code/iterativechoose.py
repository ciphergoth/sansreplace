def sorted_choose(n, k):
    d = []
    for i in range(n):
        if random.randrange(n - i) < k - len(d):
            d.append(i)
    return d
