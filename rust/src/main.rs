use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use rand::{rngs::SmallRng, Rng, SeedableRng};

fn quadratic_reject<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: Rng + ?Sized,
{
    let amount = res.len();
    for i in 0..amount {
        loop {
            let t = rng.gen_range(0..length);
            if !res[..i].contains(&t) {
                res[i] = t;
                break;
            }
        }
    }
}

fn quadratic_f2<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: Rng + ?Sized,
{
    let amount = res.len();
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.gen_range(0..=j);
        if let Some(pos) = res[..i].iter().position(|&x| x == t) {
            res[pos] = j;
        }
        res[i] = t;
    }
}

fn reject<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: Rng + ?Sized,
{
    let amount = res.len();
    let mut done = std::collections::HashSet::with_capacity(amount);
    for i in 0..amount {
        loop {
            let t = rng.gen_range(0..length);
            if !done.contains(&t) {
                res[i] = t;
                done.insert(t);
                break;
            }
        }
    }
}

fn time_test<F>(f: F, length: usize, amount: usize) -> std::time::Duration
where
    F: Fn(&mut SmallRng, usize, &mut [usize]),
{
    assert!(amount <= length);
    let mut rng = SmallRng::from_entropy();
    let mut v = vec![0; amount];
    let mut iters = 1;
    let mut remaining = std::time::Duration::from_millis(500);
    loop {
        let now = Instant::now();
        for _ in 0..iters {
            f(&mut rng, length, &mut v);
            black_box(&v);
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

fn main() {
    let totest: &[(&str, Box<dyn Fn(&mut SmallRng, usize, &mut [usize])>)] = &[
        ("reject", Box::new(reject)),
        ("quadratic_f2", Box::new(quadratic_f2)),
        ("quadratic_reject", Box::new(quadratic_reject)),
    ];
    const MAX: usize = 1_000;
    const MAXTIME: Duration = Duration::from_secs(1);
    for (name, f) in totest.iter() {
        let mut kk = (1, 1);
        'kk: while kk.1 < MAX {
            let mut nn = kk;
            while nn.1 < MAX {
                let t = time_test(f, nn.1, kk.1);
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
}
