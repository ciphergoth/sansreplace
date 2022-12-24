import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

def plot(toplot, data, title, output):
    if output is not None:
        import matplotlib
        matplotlib.use('Agg')
    fig = plt.figure()
    #plt.title(args.title)
    ax = plt.gca()
    ax.set_xscale('log')
    ax.set_yscale('linear')
    ax.set_xlabel("k")
    ax.set_ylabel("time/k (ns)")
    ax.set_xlim(1, 1E8)
    ax.set_ylim(0, 2E2)
    #ttl = ax.set_title(args.title, animated=True)
    ttl = ax.text(.5, 1.01, title, 
        horizontalalignment='center', transform = ax.transAxes)
    artists = [ttl]
    lines = []
    for alg in toplot:
        line, = ax.plot([], [])
        line.set_label(alg)
        artists.append(line)
        lines.append((alg, line))
    ax.legend()
    artists = [l[1] for l in lines]
    def init():
        ttl.set_text(title)
        for _, line in lines:
            line.set_data([], [])
        return artists
    def animate(ix):
        n, d = data[ix]
        if n < 100:
            ttl.set_text(f"{title} n={n}")
        else:
            ttl.set_text(f"{title} n={n:.1E}")
        for alg, line in lines:
            if alg not in d:
                line.set_data([], [])
            else:
                points = d[alg]
                line.set_data([p[0] for p in points], [p[1] for p in points])
        return artists
    ani = FuncAnimation(fig, animate, frames=list(range(len(data))),
                    init_func=init)
    if output is None:
        plt.show()
    else:
        ani.save(output, dpi=300)