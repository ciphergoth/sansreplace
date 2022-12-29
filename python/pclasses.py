import math

import numpy as np

def num2tex(f):
    mant, exp = f"{f:.3E}".split("E")
    exp = int(exp)
    if exp == 0:
        return mant
    return f"{mant} \\times 10^{{{exp}}}"


class One:
    def val(self, _item): return 1

    def vector(self, n, _k): return np.ones(len(n))

    def tex(self, c): return num2tex(c)

class N:
    def val(self, item): return item.n

    def vector(self, n, _k): return n

    def tex(self, c): 
        c = num2tex(c)
        return f"({c})n"

class K:
    def val(self, item): return item.k

    def vector(self, _n, k): return k

    def tex(self, c): 
        c = num2tex(c)
        return f"({c})k"

class K2:
    def val(self, item): return item.k**2

    def vector(self, _n, k): return np.square(k)

    def tex(self, c): 
        c = num2tex(c)
        return f"({c})k^2"

class KLogK:
    def val(self, item): return item.k * math.log(item.k)

    def vector(self, _n, k): return np.multiply(k, np.log(k))

    def tex(self, c): 
        c = num2tex(c)
        return f"({c})(k \\log k)"


def lookup_model(m):
    if m == '1': return One()
    elif m == 'n': return N()
    elif m == 'k': return K()
    elif m == 'k2': return K2()
    elif m == 'klogk': return KLogK()
    else:
        raise Exception(f"Unknown method: {m}")

def lookup_models(models):
    if models is None:
        yield from [K2(), KLogK(), K(), N()]
    else:
        for m in models:
            yield lookup_model(m)
    yield One()
