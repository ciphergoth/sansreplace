//! Timing, as in cpp/timing.cpp

use std::{alloc, time::Instant};

use rand::SeedableRng;

use crate::{
    Algorithm, Rng,
    counters::{Counters, Counts},
};

pub struct Point {
    pub function: Algorithm,
    pub n: u32,
    pub k: u32,
    pub seed: u64,
}

pub struct Budget {
    /// Time each stack position for at least this long, in seconds, and over
    /// at least this many batches of calls
    pub min_time: f64,
    pub min_batches: usize,
    /// How many stack positions to time at, 16 bytes apart; a point whose
    /// call takes min_time or longer is timed at the first only
    pub positions: usize,
    /// A point whose first call takes this long, in seconds, is timed by that
    /// call alone
    pub once_above: f64,
}

const STACK_SIZE: usize = 8 << 20;
// The largest page we run with is the 16 KB of an Apple Silicon Mac
const PAGE: usize = 16384;

/// Run f on a stack of our own, whose top is pad bytes below a page boundary;
/// pad must be a multiple of 16. Where the stack the process starts on lies
/// depends on its environment, its arguments and the kernel's randomisation,
/// and where the stack lies within its page and its cache line changes some
/// algorithms' times by 20% or more: see measurement.md.
fn on_stack<T>(base: *mut u8, pad: usize, f: impl FnOnce() -> T) -> T {
    unsafe { psm::on_stack(base, STACK_SIZE - pad, f) }
}

fn new_stack() -> *mut u8 {
    unsafe { alloc::alloc(alloc::Layout::from_size_align(STACK_SIZE, PAGE).unwrap()) }
}

/// The output starts on a cache line, since malloc only promises 16 bytes,
/// and the quadratic algorithms scan it on every draw: how many of their
/// loads and stores straddle two cache lines would otherwise depend on where
/// malloc happened to put it. Vec can't ask for more alignment, so this has
/// 15 spare elements, and the output starts at the first one on a 64-byte
/// boundary.
fn make_result(k: u32) -> (Vec<u32>, usize) {
    let buf = vec![0; k as usize + 15];
    let offset = buf.as_ptr().align_offset(64);
    (buf, offset)
}

fn minor_faults() -> i64 {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    unsafe {
        libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr());
        usage.assume_init().ru_minflt
    }
}

/// The time iters calls take, in seconds, and what the counters counted
fn batch(
    point: &Point,
    rng: &mut Rng,
    result: &mut [u32],
    iters: u64,
    counters: &Counters,
    counts: &mut Counts,
) -> f64 {
    let mut before = Counts::default();
    let mut after = Counts::default();
    counters.read(&mut before);
    let start = Instant::now();
    for _ in 0..iters {
        (point.function)(rng, point.n, result);
        std::hint::black_box(&result);
    }
    let t = start.elapsed().as_secs_f64();
    counters.read(&mut after);
    for i in 0..counters.size() {
        counts.value[i] = after.value[i] - before.value[i];
    }
    t
}

/// Every batch starts from the same seed, so that every batch of a point
/// makes the same draws, in any run
fn seeded_batch(
    point: &Point,
    result: &mut [u32],
    iters: u64,
    counters: &Counters,
    counts: &mut Counts,
) -> f64 {
    let mut rng = Rng::seed_from_u64(point.seed);
    batch(point, &mut rng, result, iters, counters, counts)
}

fn print_position(
    pad: usize,
    iters: u64,
    times: &[f64],
    fastest: f64,
    faults: i64,
    counters: &Counters,
    counts: &Counts,
) {
    let mut sorted = times.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 1 {
        sorted[mid]
    } else {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    };
    let mut line = format!(
        "{pad} {iters} {} {} {} {faults}",
        times.len(),
        fastest / iters as f64,
        median / iters as f64
    );
    for i in 0..counters.size() {
        line += &format!(" {}", counts.value[i] as f64 / iters as f64);
    }
    println!("{line}");
}

/// Time at one stack position, and return the time per call. iters is the
/// calls in a batch: zero at the first position, which sets it, so that every
/// position times the same calls.
fn time_position(
    point: &Point,
    budget: &Budget,
    counters: &Counters,
    result: &mut [u32],
    pad: usize,
    iters: &mut u64,
) -> f64 {
    let mut times = Vec::with_capacity(64);
    let mut counts = Counts::default();
    let mut best = Counts::default();
    let mut faults = minor_faults();
    if *iters == 0 {
        let mut t = seeded_batch(point, result, 1, counters, &mut counts);
        if t >= budget.once_above {
            times.push(t);
            print_position(pad, 1, &times, t, minor_faults() - faults, counters, &counts);
            return t;
        }
        // That call, and these, also warm up
        *iters = 1;
        while t < budget.min_time / 10.0 {
            *iters *= 2;
            t = seeded_batch(point, result, *iters, counters, &mut counts);
        }
    } else {
        // Warm up
        seeded_batch(point, result, *iters, counters, &mut counts);
    }
    faults = minor_faults();
    let mut total = 0.0;
    let mut fastest = 0.0;
    while times.len() < budget.min_batches || total < budget.min_time {
        let t = seeded_batch(point, result, *iters, counters, &mut counts);
        total += t;
        if times.is_empty() || t < fastest {
            fastest = t;
            best = counts;
        }
        times.push(t);
    }
    print_position(pad, *iters, &times, fastest, minor_faults() - faults, counters, &best);
    fastest / *iters as f64
}

/// Time a point, and print a line for each stack position it was timed at:
/// the position's offset in bytes, the calls in a batch, the number of
/// batches, the time per call in the fastest batch and in the median one, the
/// minor page faults over all the batches, and what each counter counted per
/// call in the fastest batch.
pub fn time_point(point: &Point, budget: &Budget, counters: &Counters) {
    let (mut buf, offset) = make_result(point.k);
    let result = &mut buf[offset..offset + point.k as usize];
    let stack = new_stack();
    let mut iters = 0;
    for i in 0..budget.positions {
        let t = on_stack(stack, 16 * i, || {
            time_position(point, budget, counters, result, 16 * i, &mut iters)
        });
        if t >= budget.min_time {
            break;
        }
    }
}

/// Make this many calls one after another at one stack position, and print a
/// line for each: its time, and what each counter counted.
pub fn time_calls(point: &Point, calls: usize, pad: usize, counters: &Counters) {
    let (mut buf, offset) = make_result(point.k);
    let result = &mut buf[offset..offset + point.k as usize];
    let mut times = vec![0.0; calls];
    let mut counts = vec![Counts::default(); calls];
    on_stack(new_stack(), pad, || {
        let mut rng = Rng::seed_from_u64(point.seed);
        for c in 0..calls {
            times[c] = batch(point, &mut rng, result, 1, counters, &mut counts[c]);
        }
    });
    for c in 0..calls {
        let mut line = format!("{}", times[c]);
        for i in 0..counters.size() {
            line += &format!(" {}", counts[c].value[i]);
        }
        println!("{line}");
    }
}
