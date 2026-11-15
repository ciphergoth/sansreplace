//! How much memory does a program hold if it keeps many samples from the
//! rand crate? rand's sample_inplace returns its whole working vector,
//! truncated, so each kept sample holds as many bytes as four times the
//! length it was drawn from.
//!
//!   retained SAMPLER LENGTH AMOUNT COUNT
//!
//! SAMPLER is inplace, for rand's sample_inplace; inplaceseparate, for the
//! version with a working vector of its own; or inplaceshrink, for the version
//! that shrinks its vector to the result. Draws COUNT samples of AMOUNT
//! indices from 0..LENGTH, keeps them all, and prints the bytes the samples
//! need, the bytes their vectors have capacity for, and how much the
//! process's resident memory grew, which is what the allocator really holds.
//! Linux only, since it reads /proc/self/statm.
//!
//! Needs the rand_internals feature and a checkout of rand that exposes its
//! samplers: see main.rs.

use rand::SeedableRng;
use rand::seq::index::{IndexVec, internals};

/// The process's resident memory, in bytes
fn resident() -> usize {
    let statm = std::fs::read_to_string("/proc/self/statm").unwrap();
    let pages: usize = statm.split_whitespace().nth(1).unwrap().parse().unwrap();
    pages * 4096
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sampler = match args[1].as_str() {
        "inplace" => internals::inplace,
        "inplaceseparate" => internals::inplace_separate,
        "inplaceshrink" => internals::inplace_shrink,
        other => panic!("unknown sampler {other}"),
    };
    let length: u32 = args[2].parse().unwrap();
    let amount: u32 = args[3].parse().unwrap();
    let count: usize = args[4].parse().unwrap();
    let mut rng = rand_pcg::Pcg32::seed_from_u64(123);
    let mut kept = Vec::with_capacity(count);
    let before = resident();
    for _ in 0..count {
        kept.push(sampler(&mut rng, length, amount));
    }
    let after = resident();
    let capacity: usize = kept
        .iter()
        .map(|sample| match sample {
            IndexVec::U32(v) => v.capacity() * 4,
            IndexVec::U64(v) => v.capacity() * 8,
        })
        .sum();
    println!(
        "{} {amount} of {length} x {count}: needed {} bytes, capacity {capacity} bytes, resident memory grew {} bytes",
        args[1],
        count * amount as usize * 4,
        after - before
    );
}
