use std::{collections::BTreeMap, path::Path, time::Duration};

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use rand::rngs::SmallRng;

mod algorithms;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, global = true, value_enum, default_value_t = algorithms::SortType::Random)]
    sorttype: algorithms::SortType,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Time every algorithm over a range of n, k values, writing the results to a CSV file
    Sweep { csv: std::path::PathBuf },
    /// Time every algorithm for a single n, k
    Time { n: u32, k: u32 },
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

fn time_test<F>(f: F, length: u32, amount: u32) -> std::time::Duration
where
    F: Fn(&mut SmallRng, u32, &mut [u32]),
{
    assert!(amount <= length);
    let mut rng = rand::make_rng::<SmallRng>();
    let mut v = vec![0; amount as usize];
    let mut iters = 1;
    let mut remaining = std::time::Duration::from_millis(500);
    loop {
        let now = std::time::Instant::now();
        for _ in 0..iters {
            f(&mut rng, length, &mut v);
            std::hint::black_box(&v);
        }
        let t = now.elapsed();
        if remaining < t {
            return t.checked_div(iters).unwrap();
        }
        remaining -= t;
        if remaining < t {
            return t.checked_div(iters).unwrap();
        }
        let ratio = remaining.as_secs_f64() / t.as_secs_f64();
        if ratio <= 60.0 {
            iters = ((iters as f64) * ratio) as u32;
        } else {
            iters *= 3;
        }
    }
}

fn write_row(path: &Path, name: &str, n: u32, k: u32, t: Duration) -> Result<()> {
    let f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("Opening file {}", path.display()))?;
    let mut wtr = csv::WriterBuilder::new().from_writer(f);
    wtr.serialize(("result", name, n, k, t.as_secs_f64()))?;
    Ok(())
}

fn time_all(sort_type: algorithms::SortType, csv: &Path) -> Result<()> {
    const MAX: u32 = 1_000_000_000;
    const MAXTIME: Duration = Duration::from_secs(3);
    for (name, f) in algorithms::algorithms(sort_type).iter() {
        let mut kk = (1, 1);
        'kk: while kk.1 < MAX {
            let mut nn = kk;
            while nn.1 < MAX {
                let t = time_test(f, nn.1, kk.1);
                write_row(csv, name, nn.1, kk.1, t)?;
                println!("{} {} {}: {:?}", name, nn.1, kk.1, t);
                if t > MAXTIME {
                    if nn == kk {
                        break 'kk;
                    } else {
                        break;
                    }
                }
                nn = (nn.1, nn.0 + nn.1);
            }
            kk = (kk.1, kk.0 + kk.1);
        }
    }
    Ok(())
}

fn time_one(sort_type: algorithms::SortType, n: u32, k: u32) {
    let mut res: Vec<_> = algorithms::algorithms(sort_type)
        .iter()
        .map(|(name, f)| (time_test(f, n, k), *name))
        .collect();
    res.sort();
    for (t, name) in res {
        println!("{t:>12.1?} {name}");
    }
}

fn binomial(n: u32, k: u32) -> Result<u128> {
    let mut r: u128 = 1;
    for i in 0..k.min(n - k) {
        r = r
            .checked_mul((n - i).into())
            .ok_or_else(|| anyhow!("binomial({n}, {k}) overflows"))?;
        r /= u128::from(i + 1);
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
        Command::Sweep { csv } => {
            if csv.is_file() {
                std::fs::remove_file(&csv)?;
            }
            time_all(args.sorttype, &csv)?;
        }
        Command::Time { n, k } => time_one(args.sorttype, n, k),
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
