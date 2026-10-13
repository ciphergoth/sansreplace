use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result, anyhow, ensure};
use clap::{Parser, Subcommand};
use rand::rngs::SmallRng;

mod algorithms;
mod nkvalues;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, global = true, value_enum, default_value_t = algorithms::SortType::Sorted)]
    sorttype: algorithms::SortType,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Time multiple algorithms against multiple n,k values, like python/time_choose
    Time {
        /// Maximum time to iterate a given test
        #[arg(long, default_value_t = 0.1)]
        time_for: f64,
        /// Maximum time a single test can run
        #[arg(long, default_value_t = 0.1)]
        time_limit: f64,
        /// Algorithm to time (default: all)
        #[arg(long)]
        algorithm: Vec<String>,
        /// Named set of n,k values to time (default: small and power10)
        #[arg(long, value_enum, conflicts_with = "nk")]
        values: Vec<nkvalues::ValueSet>,
        /// Single n,k value to time
        #[arg(long, num_args = 2, value_names = ["N", "K"])]
        nk: Vec<u32>,
        /// Skip n,k values with k larger than this
        #[arg(long)]
        max_k: Option<u32>,
        /// File to write results to
        #[arg(long)]
        csv: Option<PathBuf>,
    },
    /// Test one algorithm for fairness by counting how often each output occurs
    Test {
        n: u32,
        k: u32,
        /// Number of tests to run
        #[arg(long, default_value_t = 100_000)]
        num_tests: u64,
        /// Maximum number of result rows to show
        #[arg(long, default_value_t = 21)]
        show_max: usize,
        /// Algorithm to test
        #[arg(long, default_value = "cardchoose")]
        algorithm: String,
    },
}

type Rng = SmallRng;

/// Total time for iters calls, like timefunc_s in cpp/timing.cpp
fn time_iters(
    f: &dyn Fn(&mut Rng, u32, &mut [u32]),
    rng: &mut Rng,
    n: u32,
    k: u32,
    iters: u64,
) -> f64 {
    let mut result = vec![0; k as usize];
    let start = Instant::now();
    for _ in 0..iters {
        f(rng, n, &mut result);
        std::hint::black_box(&result);
    }
    start.elapsed().as_secs_f64()
}

/// Time per call, like timefunc_for in python/time_choose
fn timefunc_for(
    time: f64,
    f: &dyn Fn(&mut Rng, u32, &mut [u32]),
    rng: &mut Rng,
    n: u32,
    k: u32,
) -> f64 {
    let mut iters: u64 = 1;
    let mut remaining = time;
    loop {
        let t = time_iters(f, rng, n, k, iters);
        remaining -= t;
        if t >= remaining {
            return t / iters as f64;
        } else if t * 100.0 > remaining {
            iters = (iters as f64 * remaining / t) as u64;
        } else if t * 10000.0 > remaining {
            iters = (iters as f64 * remaining * 0.1 / t) as u64;
        } else {
            iters *= 10;
        }
    }
}

/// Like python/gitstate.py
fn gitstate(topdir: &Path) -> Result<String> {
    let rp = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(topdir)
        .output()?;
    anyhow::ensure!(rp.status.success(), "git rev-parse failed");
    let mut state = String::from_utf8(rp.stdout)?.trim().to_string();
    // diff-index compares file stats, so refresh them first, or a file whose
    // timestamp changed without its contents changing counts as dirty
    std::process::Command::new("git")
        .args(["update-index", "-q", "--refresh"])
        .current_dir(topdir)
        .status()?;
    let di = std::process::Command::new("git")
        .args(["diff-index", "--quiet", "HEAD", "--"])
        .current_dir(topdir)
        .status()?;
    if !di.success() {
        state += "+";
    }
    Ok(state)
}

/// Format like Python's repr of a list of strings, or empty for an unset option
fn pylist<S: std::fmt::Display>(items: &[S]) -> String {
    if items.is_empty() {
        return String::new();
    }
    let items: Vec<_> = items.iter().map(|i| format!("'{i}'")).collect();
    format!("[{}]", items.join(", "))
}

struct TimeArgs {
    sort_type: algorithms::SortType,
    time_for: f64,
    time_limit: f64,
    algorithm: Vec<String>,
    values: Vec<nkvalues::ValueSet>,
    nk: Vec<u32>,
    max_k: Option<u32>,
    csv: Option<PathBuf>,
}

fn time_choose(args: TimeArgs) -> Result<()> {
    let topdir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut timeables = algorithms::algorithms::<Rng>(args.sort_type);
    if !args.algorithm.is_empty() {
        let mut by_name: HashMap<_, _> = timeables.into_iter().collect();
        timeables = args
            .algorithm
            .iter()
            .map(|a| {
                by_name
                    .remove_entry(a.as_str())
                    .ok_or_else(|| anyhow!("Unknown algorithm: {a}"))
            })
            .collect::<Result<_>>()?;
    }
    for c in args.nk.chunks(2) {
        ensure!(c[1] <= c[0], "--nk {} {}: k must be at most n", c[0], c[1]);
    }
    let default_values = [nkvalues::ValueSet::Small, nkvalues::ValueSet::Power10];
    let values: Box<dyn Iterator<Item = (String, u32, u32)>> = if !args.nk.is_empty() {
        Box::new(
            args.nk
                .chunks(2)
                .map(|c| (format!("n = {}, k = {}", c[0], c[1]), c[0], c[1])),
        )
    } else if !args.values.is_empty() {
        Box::new(nkvalues::iter_values(&args.values))
    } else {
        Box::new(nkvalues::iter_values(&default_values))
    };
    let mut rng = rand::make_rng::<Rng>();
    let mut writer = match &args.csv {
        None => None,
        Some(csv) => {
            if let Some(parent) = csv.parent() {
                std::fs::create_dir_all(parent)?;
            }
            // Rows have different lengths, as with Python's csv module
            let mut w = csv::WriterBuilder::new()
                .flexible(true)
                .from_path(csv)
                .with_context(|| format!("Opening file {}", csv.display()))?;
            let config = [
                ("algorithm", pylist(&args.algorithm)),
                ("csv", csv.display().to_string()),
                ("language", "rust".to_string()),
                ("max_k", args.max_k.map_or(String::new(), |m| m.to_string())),
                (
                    "nk",
                    if args.nk.is_empty() {
                        String::new()
                    } else {
                        format!("{:?}", args.nk.chunks(2).collect::<Vec<_>>())
                    },
                ),
                ("sorttype", format!("{:?}", args.sort_type).to_lowercase()),
                ("time_for", args.time_for.to_string()),
                ("time_limit", args.time_limit.to_string()),
                (
                    "values",
                    pylist(
                        &args
                            .values
                            .iter()
                            .map(|v| format!("{v:?}").to_lowercase())
                            .collect::<Vec<_>>(),
                    ),
                ),
            ];
            for (k, v) in config {
                w.write_record(["config", k, &v])?;
            }
            let donothing = |_: &mut Rng, _: u32, _: &mut [u32]| {};
            let info = [
                ("version", gitstate(topdir)?),
                (
                    "donothing_s",
                    format!(
                        "{:e}",
                        timefunc_for(args.time_for, &donothing, &mut rng, 0, 0)
                    ),
                ),
                (
                    "algorithms",
                    pylist(&timeables.iter().map(|(a, _)| *a).collect::<Vec<_>>()),
                ),
            ];
            for (k, v) in info {
                w.write_record(["info", k, &v])?;
            }
            w.flush()?;
            Some(w)
        }
    };
    let mut klim: HashMap<&str, u32> = HashMap::new();
    for (descrip, n, k) in values.filter(|v| args.max_k.is_none_or(|m| v.2 <= m)) {
        println!("{descrip}");
        let mut res = Vec::new();
        for (alg, f) in &timeables {
            if klim.get(alg).is_none_or(|&l| k < l) {
                let t = timefunc_for(args.time_for, f, &mut rng, n, k);
                if let Some(w) = &mut writer {
                    w.write_record([
                        "result",
                        alg,
                        &n.to_string(),
                        &k.to_string(),
                        &format!("{t:e}"),
                    ])?;
                    w.flush()?;
                }
                res.push((t, *alg));
                if t > args.time_limit {
                    println!("hit tlim: {alg}");
                    klim.insert(alg, k);
                }
            }
        }
        if res.is_empty() {
            println!();
            continue;
        }
        res.sort_by(|a, b| a.0.total_cmp(&b.0));
        let (unit, mul) = if res[0].0 < 1E-6 {
            ("ns", 1E9)
        } else if res[0].0 < 1E-3 {
            ("μs", 1E6)
        } else if res[0].0 < 1.0 {
            ("ms", 1E3)
        } else {
            ("s", 1.0)
        };
        for (t, alg) in res {
            println!("{:12.1}{unit} {alg:20}", t * mul);
        }
        println!();
    }
    Ok(())
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn binomial(n: u32, k: u32) -> Result<u128> {
    // C(n, i+1) = C(n, i) (n-i) / (i+1). Cancel what C(n, i) shares with
    // i+1 first; what's left of i+1 then divides n-i, so no intermediate
    // value exceeds the result.
    let mut r: u128 = 1;
    for i in 0..k.min(n - k) {
        let g = gcd(r, u128::from(i + 1));
        let d = u128::from(i + 1) / g;
        r = (r / g)
            .checked_mul(u128::from(n - i) / d)
            .ok_or_else(|| anyhow!("binomial({n}, {k}) overflows"))?;
    }
    Ok(r)
}

fn test_choose(
    sort_type: algorithms::SortType,
    n: u32,
    k: u32,
    num_tests: u64,
    show_max: usize,
    algorithm: &str,
) -> Result<()> {
    ensure!(k <= n, "k must be at most n");
    let (_, f) = algorithms::algorithms(sort_type)
        .into_iter()
        .find(|(name, _)| *name == algorithm)
        .ok_or_else(|| anyhow!("Unknown algorithm: {algorithm}"))?;
    let mut rng = rand::make_rng::<SmallRng>();
    let mut v = vec![0; k as usize];
    let mut counts: BTreeMap<Vec<u32>, u64> = BTreeMap::new();
    for _ in 0..num_tests {
        f(&mut rng, n, &mut v);
        *counts.entry(v.clone()).or_default() += 1;
    }
    if counts.len() <= show_max {
        for (r, c) in &counts {
            println!("{r:?}: {c}");
        }
    }
    println!("{} {}", counts.len(), binomial(n, k)?);
    println!(
        "{} {}",
        counts.values().min().unwrap(),
        counts.values().max().unwrap()
    );
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Time {
            time_for,
            time_limit,
            algorithm,
            values,
            nk,
            max_k,
            csv,
        } => time_choose(TimeArgs {
            sort_type: args.sorttype,
            time_for,
            time_limit,
            algorithm,
            values,
            nk,
            max_k,
            csv,
        })?,
        Command::Test {
            n,
            k,
            num_tests,
            show_max,
            algorithm,
        } => test_choose(args.sorttype, n, k, num_tests, show_max, &algorithm)?,
    }
    Ok(())
}
