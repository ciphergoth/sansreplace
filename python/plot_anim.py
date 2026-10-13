import matplotlib
import matplotlib.lines
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

# The line style shows each algorithm's family, so the graphs can be read
# without telling twelve colours apart, and the colour tells the algorithms
# in a family apart. Dotted lines are hard to see, so they are drawn thicker.
# The legend gives each family a column.
FAMILIES = [
    ({"linestyle": "-"}, {             # no extra memory, O(k) or O(k log k)
        "ranksb": "tab:blue",
        "vitterd": "tab:orange",
        "hiddenshuffle": "tab:green",
        "cardchoose": "tab:red",
    }),
    ({"linestyle": "--"}, {            # hash-based
        "rejectionsample": "tab:purple",
        "floydf2": "tab:brown",
        "hsel": "tab:pink",
    }),
    ({"linestyle": ":", "linewidth": 2.2}, {  # quadratic
        "quadraticreject": "tab:gray",
        "quadraticf2": "tab:olive",
    }),
    ({"linestyle": "-."}, {            # O(n)
        "iterativechoose": "tab:cyan",
        "reservoirsample": "black",
        "select": "darkgoldenrod",
    }),
]
STYLES = {alg: {**style, "color": colour}
    for style, colours in FAMILIES for alg, colour in colours.items()}

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

def legend(fig, lines):
    "A column per family; the legend fills columns first, so pad each to the same height"
    columns = [[lines[alg] for alg in colours if alg in lines] for _, colours in FAMILIES]
    height = max(len(c) for c in columns)
    blank = matplotlib.lines.Line2D([], [], linestyle="none", label="")
    handles = [h for c in columns for h in c + [blank] * (height - len(c))]
    fig.legend(handles=handles, loc="outside lower center", ncols=len(columns),
        fontsize="small", frameon=False)

def _plot(toplot, data, title, output):
    # Taller than usual, to leave room for the legend below
    width, height = plt.rcParams["figure.figsize"]
    fig = plt.figure(layout="constrained", figsize=(width, height * 1.25))
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
    legend(fig, dict(lines))
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
