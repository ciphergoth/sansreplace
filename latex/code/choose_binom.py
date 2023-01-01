def choose_binom(n, k):
    m = choose_multiset(n - k + 1, k)
    m.sort()
    for i in range(k):
        m[i] += i
    return m
