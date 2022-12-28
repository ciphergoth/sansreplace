import math

import numpy as np

class One:
    def val(self, _item): return 1

    def vector(self, n, _k): return np.ones(len(n))

class N:
    def val(self, item): return item.n

    def vector(self, n, _k): return n

class K:
    def val(self, item): return item.k

    def vector(self, _n, k): return k

class K2:
    def val(self, item): return item.k**2

    def vector(self, _n, k): return np.square(k)

class KLogK:
    def val(self, item): return item.k * math.log(item.k)

    def vector(self, _n, k): return np.multiply(k, np.log(k))

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
        yield from [One(), N(), K(), K2(), KLogK()]
    yield One()
    for m in models:
        yield lookup_model(m)