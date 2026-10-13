def sorted_choose(n, k):
    lo = lambda l: l * n // k
    counts = [0] * k
    accepted = 0
    while accepted < k:
        x = random.randrange(n)
        l = ((x + 1) * k - 1) // n
        if x - lo(l) >= counts[l]:
            counts[l] += 1
            accepted += 1
    d = []
    for l in range(k):
        s = []
        while len(s) < counts[l]:
            x = random.randrange(lo(l), lo(l + 1))
            if x not in s:
                s.append(x)
        s.sort()
        d.extend(s)
    return d
