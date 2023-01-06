use std::{hint::black_box, time::Instant};

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

fn time_test<F>(f: F, length: usize, amount: usize) -> std::time::Duration
where
    F: Fn(&mut SmallRng, usize, &mut [usize]),
{
    assert!(amount <= length);
    let mut rng = SmallRng::from_entropy();
    let mut v = vec![0; amount];
    let mut iters = 1;
    let mut remaining = std::time::Duration::from_secs(2);
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
    let totest: &[(&str, Box<dyn Fn(&mut SmallRng, usize, &mut [usize])>)]
     = &[
        ("quadratic_f2", Box::new(quadratic_f2)), 
        ("quadratic_reject", Box::new(quadratic_reject)),
    ];
    let mut nn = (1, 1);
    while nn.1 < 1_000_000_000 {
        let mut kk = (1, 1);
        while kk.1 <= nn.1 {
            for (name, f) in totest.iter() {
                let t = time_test(f, nn.1, kk.1);
                println!("{} {} {}: {:?}", name, nn.1, kk.1, t);
            }
            kk = (kk.1, kk.0 + kk.1);
        } 
        nn = (nn.1, nn.0 + nn.1);
    }
}
