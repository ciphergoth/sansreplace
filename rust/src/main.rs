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
//! components.rs, or, for the one that does nothing, donothing. In this
//! program alone it can also be rand, for the rand crate's own sampler. See
//! timing.rs for what time and calls print.

use clap::{Args, Parser, Subcommand};
use rand::SeedableRng;

mod algorithms;
mod components;
mod counters;
mod timing;

/// SmallRng is Xoshiro256++, and like rand_init in cpp/randbelow.c,
/// seed_from_u64 expands the seed with SplitMix64, so both start from the same
/// state.
#[cfg(not(any(feature = "pcg32", feature = "stdrng")))]
type Rng = rand::rngs::SmallRng;
/// With the pcg32 feature, the generator that the rand crate's own sequence
/// benchmarks use
#[cfg(feature = "pcg32")]
type Rng = rand_pcg::Pcg32;
/// With the stdrng feature, the generator behind rand::rng(), which is what a
/// program gets unless it chooses another: ChaCha with 12 rounds
#[cfg(feature = "stdrng")]
type Rng = rand::rngs::StdRng;
type Algorithm = algorithms::Algorithm<Rng>;

const ORDERS: [&str; 5] = ["donothing", "random", "sorted", "component", "rand"];

fn table(order: &str) -> Vec<(&'static str, Algorithm)> {
    match order {
        "donothing" => vec![(
            "donothing",
            Box::new(|_: &mut Rng, _: u32, _: &mut [u32]| {}) as Algorithm,
        )],
        "random" => algorithms::algorithms(algorithms::SortType::Random),
        "sorted" => algorithms::algorithms(algorithms::SortType::Sorted),
        "component" => components::components(),
        // What the rand crate itself does when asked for a sample, which is
        // in random order: it chooses between algorithms of its own. Built
        // against a checkout of rand, this times a change to it as python/bench
        // times everything else. It returns a vector, so copy that to the
        // output, as a caller that wants the indices somewhere must.
        "rand" => rand_samplers(),
        _ => panic!("unknown order {order}"),
    }
}

/// Copy the vector that one of rand's samplers returns to the output
fn copy_out(sample: rand::seq::index::IndexVec, res: &mut [u32]) {
    for (r, index) in res.iter_mut().zip(sample) {
        *r = index as u32;
    }
}

fn rand_samplers() -> Vec<(&'static str, Algorithm)> {
    #[allow(unused_mut)]
    let mut samplers = vec![(
        "indexsample",
        Box::new(|rng: &mut Rng, length: u32, res: &mut [u32]| {
            copy_out(
                rand::seq::index::sample(rng, length as usize, res.len()),
                res,
            )
        }) as Algorithm,
    )];
    // With the rand_internals feature, each of the algorithms that
    // index::sample chooses between, so that each can be timed where sample
    // would not choose it. These need a checkout of rand that exposes them,
    // which the published crate doesn't.
    #[cfg(feature = "rand_internals")]
    {
        use rand::seq::index::internals;
        type Sampler = fn(&mut Rng, u32, u32) -> rand::seq::index::IndexVec;
        let each: [(&'static str, Sampler); 9] = [
            ("floyd", internals::floyd),
            ("inplace", internals::inplace),
            ("rejection", internals::rejection),
            // Rejection sampling as rand does it without std
            ("rejectionbtree", internals::rejection_btree),
            ("cardchoose", internals::cardchoose),
            // Changes we might propose, each returning what the original does:
            // Floyd's with a scan that vectorises, and inplace with a working
            // vector apart from the one it returns, or shrinking the one it
            // returns, or returning a copy of the result
            ("floydvectorised", internals::floyd_vectorised),
            ("inplaceseparate", internals::inplace_separate),
            ("inplaceshrink", internals::inplace_shrink),
            ("inplacecopy", internals::inplace_copy),
        ];
        for (name, sampler) in each {
            samplers.push((
                name,
                Box::new(move |rng: &mut Rng, length: u32, res: &mut [u32]| {
                    copy_out(sampler(rng, length, res.len() as u32), res)
                }) as Algorithm,
            ));
        }
    }
    samplers
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
