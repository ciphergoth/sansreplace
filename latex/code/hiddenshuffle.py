def sorted_choose(n, k):
    d = []
    h = 0
    if 0 < k < n:
        h = k
        q = k / n
        i = 0
        while True:
            i += int(math.log(1 - random.random())
                     / math.log1p(-q))
            if i >= k:
                break
            if random.random() * q < (k - i) / (n - i):
                h -= 1
            i += 1
    low = k - h
    a = 1.0
    prev = n - k
    while h > 0:
        a *= random.random() ** (1 / h)
        s = int(a * (n - k))
        if s < prev:
            d.append(n - 1 - (k + s))
            prev = s
        else:
            low += 1
        h -= 1
    m = k
    while low > 0:
        u = random.random()
        s = 0
        f = low / m
        while f < u and s < m - low:
            f = 1 - (1 - low / (m - s - 1)) * (1 - f)
            s += 1
        m -= s + 1
        low -= 1
        d.append(n - 1 - m)
    return d
