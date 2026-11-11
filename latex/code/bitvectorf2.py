def sorted_choose(n, k):
    bits = bytearray(n)
    for m in range(n - k, n):
        j = random.randrange(m + 1)
        if bits[j]:
            bits[m] = 1
        else:
            bits[j] = 1
    return [x for x in range(n) if bits[x]]
