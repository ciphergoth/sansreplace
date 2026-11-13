//! The Rust implementations, as a program that python/bench runs once for
//! each point it times, and python/test_choose runs for samples, with the
//! same command line as the C++ one in cpp/main.cpp:
//!
//!   sansreplace list
//!   sansreplace sample --order ORDER --algorithm NAME --n N --k K --seed SEED --count COUNT
//!   sansreplace time --order ORDER --algorithm NAME --n N --k K --seed SEED ...
//!   sansreplace calls --order ORDER --algorithm NAME --n N --k K --seed SEED --calls CALLS ...
//!
//! ORDER is random, sorted, component for the parts of the algorithms in
//! components.rs, or, for the one that does nothing, donothing. See timing.rs
//! for what time and calls print.

use clap::{Args, Parser, Subcommand};
use rand::{SeedableRng, rngs::SmallRng};

mod algorithms;
mod components;
mod counters;
mod timing;

/// SmallRng is Xoshiro256++, and like rand_init in cpp/randbelow.c,
/// seed_from_u64 expands the seed with SplitMix64, so both start from the same
/// state.
type Rng = SmallRng;
type Algorithm = algorithms::Algorithm<Rng>;

const ORDERS: [&str; 4] = ["donothing", "random", "sorted", "component"];

fn table(order: &str) -> Vec<(&'static str, Algorithm)> {
    match order {
        "donothing" => vec![(
            "donothing",
            Box::new(|_: &mut Rng, _: u32, _: &mut [u32]| {}) as Algorithm,
        )],
        "random" => algorithms::algorithms(algorithms::SortType::Random),
        "sorted" => algorithms::algorithms(algorithms::SortType::Sorted),
        "component" => components::components(),
        _ => panic!("unknown order {order}"),
    }
}

/// Sample without replacement, or time doing it
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Args)]
struct PointArgs {
    #[arg(long)]
    order: String,
    #[arg(long)]
    algorithm: String,
    #[arg(long)]
    n: u32,
    #[arg(long)]
    k: u32,
    #[arg(long)]
    seed: u64,
}

impl PointArgs {
    fn point(&self) -> timing::Point {
        assert!(self.k <= self.n, "k must be at most n");
        let mut table = table(&self.order);
        let i = table
            .iter()
            .position(|(name, _)| *name == self.algorithm)
            .unwrap_or_else(|| panic!("no {} {}", self.order, self.algorithm));
        timing::Point {
            function: table.swap_remove(i).1,
            n: self.n,
            k: self.k,
            seed: self.seed,
        }
    }
}

#[derive(Args)]
struct CounterArgs {
    /// NAME=TYPE:CONFIG, a performance counter as perf_event_open takes it,
    /// with the config in hex; may be repeated
    #[cfg(target_os = "linux")]
    #[arg(long = "counter")]
    events: Vec<String>,
}

impl CounterArgs {
    fn counters(&self) -> counters::Counters {
        #[cfg(target_os = "linux")]
        return counters::Counters::new(&self.events);
        #[cfg(not(target_os = "linux"))]
        counters::Counters::new(&[])
    }
}

#[derive(Subcommand)]
enum Command {
    /// Print the order and name of everything here
    List,
    /// Print samples, one to a line
    Sample {
        #[command(flatten)]
        point: PointArgs,
        #[arg(long)]
        count: u64,
    },
    /// Time a point, at each of several stack positions
    Time {
        #[command(flatten)]
        point: PointArgs,
        #[command(flatten)]
        counters: CounterArgs,
        #[arg(long)]
        min_time: f64,
        #[arg(long)]
        min_batches: usize,
        #[arg(long)]
        positions: usize,
        #[arg(long)]
        once_above: f64,
    },
    /// Time each of a run of calls
    Calls {
        #[command(flatten)]
        point: PointArgs,
        #[command(flatten)]
        counters: CounterArgs,
        #[arg(long)]
        calls: usize,
        #[arg(long)]
        pad: usize,
    },
}

fn main() {
    match Cli::parse().command {
        Command::List => {
            for order in ORDERS {
                for (name, _) in table(order) {
                    println!("{order} {name}");
                }
            }
        }
        Command::Sample { point, count } => {
            let point = point.point();
            let mut rng = Rng::seed_from_u64(point.seed);
            let mut result = vec![0; point.k as usize];
            for _ in 0..count {
                (point.function)(&mut rng, point.n, &mut result);
                let line: Vec<String> = result.iter().map(u32::to_string).collect();
                println!("{}", line.join(" "));
            }
        }
        Command::Time {
            point,
            counters,
            min_time,
            min_batches,
            positions,
            once_above,
        } => {
            let budget = timing::Budget {
                min_time,
                min_batches,
                positions,
                once_above,
            };
            timing::time_point(&point.point(), &budget, &counters.counters());
        }
        Command::Calls {
            point,
            counters,
            calls,
            pad,
        } => timing::time_calls(&point.point(), calls, pad, &counters.counters()),
    }
}
