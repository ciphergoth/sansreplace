import matplotlib
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

# The line style shows each algorithm's family, so the graphs can be read
# without telling twelve colours apart, and the colour tells the algorithms
# in a family apart. Dotted lines are hard to see, so they are drawn thicker.
SOLID = {"linestyle": "-"}           # no extra memory, O(k) or O(k log k)
DASHED = {"linestyle": "--"}         # hash-based
DOTTED = {"linestyle": ":", "linewidth": 2.2}  # quadratic
DASHDOT = {"linestyle": "-."}        # O(n)
STYLES = {
    "ranksb": {**SOLID, "color": "tab:blue"},
    "vitterd": {**SOLID, "color": "tab:orange"},
    "hiddenshuffle": {**SOLID, "color": "tab:green"},
    "cardchoose": {**SOLID, "color": "tab:red"},
    "rejectionsample": {**DASHED, "color": "tab:purple"},
    "floydf2": {**DASHED, "color": "tab:brown"},
    "hsel": {**DASHED, "color": "tab:pink"},
    "quadraticreject": {**DOTTED, "color": "tab:gray"},
    "quadraticf2": {**DOTTED, "color": "tab:olive"},
    "iterativechoose": {**DASHDOT, "color": "tab:cyan"},
    "reservoirsample": {**DASHDOT, "color": "black"},
    "select": {**DASHDOT, "color": "darkgoldenrod"},
}

def plot(toplot, data, title, output):
    rc = {}
    if output is not None:
        output = str(output)
        if output.endswith(".pgf"):
            matplotlib.use("pgf")
            rc = {
                "pgf.texsystem": "lualatex",
                'font.family': 'serif',
                'text.usetex': True,
                'pgf.rcfonts': False,
                # Fit the paper's 5.35in text width
                'figure.figsize': (5.3, 4.0),
            }
        else:
            matplotlib.use('Agg')
    # Scoped so the pgf settings don't leak into later plots
    with matplotlib.rc_context(rc):
        _plot(toplot, data, title, output)

def _plot(toplot, data, title, output):
    fig = plt.figure()
    #plt.title(args.title)
    ax = plt.gca()
    ax.set_xscale('log')
    ax.set_yscale('linear')
    ax.set_xlabel("k")
    ax.set_ylabel("time/k (ns)")
    ax.set_xlim(1, 1E9)
    ax.set_ylim(0, 1E2)
    #ttl = ax.set_title(args.title, animated=True)
    ttl = ax.text(.5, 1.01, title, 
        horizontalalignment='center', transform = ax.transAxes)
    artists = [ttl]
    lines = []
    for alg in toplot:
        line, = ax.plot([], [], **STYLES[alg])
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
    if len(data) == 1:
        animate(0)
        if output is None:
            plt.show()
        else:
            plt.savefig(output)
    else:
        ani = FuncAnimation(fig, animate, frames=list(range(len(data))),
                        init_func=init)
        if output is None:
            plt.show()
        else:
            ani.save(output, writer="pillow", dpi=300)
