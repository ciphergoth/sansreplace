import math

import numpy as np

def gen_fit(items, models):
    gooditems = [item for item in items if item.k >= 10 and item.k != item.n]
    t = np.fromiter((item.time for item in gooditems), dtype=np.float64)
    n = np.fromiter((item.n for item in gooditems), dtype=np.float64)
    k = np.fromiter((item.k for item in gooditems), dtype=np.float64)
    ones = np.ones(len(gooditems))
    vstack = []
    for m in models:
        vstack.append(m.vector(n, k))
    a = np.vstack(vstack)
    trecip = np.reciprocal(t)
    a = np.multiply(a, np.tile(trecip, (len(vstack), 1)))
    #print(a.T)
    #print(t)
    fit, residuals, rank, singvals = np.linalg.lstsq(a.T, ones, rcond=-1)
    #print(math.sqrt(residuals[0]/len(gooditems)))
    return fit

def gen_plotdata(items, models, fit):
    plotdata = {}
    for item in items:
        if item.k == 0:
            continue
        nrow = plotdata.setdefault(item.n, {'predicted': [], 'actual': []})
        prediction = sum(c * m.val(item) for (c, m) in zip(fit, models))
        nrow['actual'].append((item.k, 1E9 * item.time/item.k))
        nrow['predicted'].append((item.k, 1E9 * prediction/item.k))
    for nv in plotdata.values():
        for v in nv.values():
            v.sort()
    plotdata = list(plotdata.items())
    plotdata.sort()
    return plotdata

