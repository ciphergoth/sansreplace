// Port of python/nkvalues.py; keep the two in step so results line up.

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum ValueSet {
    Small,
    Power10,
    Power2,
    Fib,
    Largen,
}

fn small_values() -> Vec<(String, u32, u32)> {
    let top = 11;
    let mut res = Vec::new();
    for n in 0..top {
        for k in 0..=n {
            res.push((format!("n = {n}, k = {k}"), n, k));
        }
    }
    res
}

fn power10_values() -> Vec<(String, u32, u32)> {
    let ptop = 9;
    let mut res = Vec::new();
    for pn in 0..ptop {
        for pk in 0..=pn {
            res.push((
                format!("n = 10^{pn}, k = 10^{pk}"),
                10u32.pow(pn),
                10u32.pow(pk),
            ));
        }
    }
    res
}

fn power2_values() -> Vec<(String, u32, u32)> {
    let ptop = 30;
    let mut res = Vec::new();
    for pn in 0..ptop {
        for pk in 0..=pn {
            res.push((format!("n = 2^{pn}, k = 2^{pk}"), 1 << pn, 1 << pk));
        }
    }
    res
}

fn fib(m: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let (mut a, mut b) = (0, 1);
    while a < m {
        res.push(a);
        (a, b) = (b, a + b);
    }
    res
}

fn fib_values() -> Vec<(String, u32, u32)> {
    let ptop = 1_000_000_000;
    let mut res = Vec::new();
    for (pn, n) in fib(ptop).into_iter().enumerate() {
        for (pk, k) in fib(n + 1).into_iter().enumerate() {
            res.push((format!("n = F_{pn} = {n}, k = F_{pk} = {k}"), n, k));
        }
    }
    res
}

fn largen_values() -> Vec<(String, u32, u32)> {
    let n = 10_000_000;
    (0..=n)
        .step_by(100_000)
        .map(|k| (format!("n = {n}, k = {k}"), n, k))
        .collect()
}

pub fn iter_values(names: &[ValueSet]) -> impl Iterator<Item = (String, u32, u32)> + '_ {
    let mut done = HashSet::new();
    names
        .iter()
        .flat_map(|name| match name {
            ValueSet::Small => small_values(),
            ValueSet::Power10 => power10_values(),
            ValueSet::Power2 => power2_values(),
            ValueSet::Fib => fib_values(),
            ValueSet::Largen => largen_values(),
        })
        .filter(move |(descrip, n, k)| {
            if done.insert((*n, *k)) {
                true
            } else {
                println!("skipping {descrip}, already done");
                false
            }
        })
}
