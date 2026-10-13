//! Hidden Shuffle, from Shekelyan and Cormode, "Sequential Random Sampling
//! Revisited: Hidden Shuffle Method" (AISTATS 2021), with the corrections
//! from the errata in the authors' repository. See
//! python/algorithms/hiddenshuffle.py for an overview.

fn hiddenshuffle<R>(rng: &mut R, n: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    let k = res.len() as u32;
    // Step 1: h is the number of the first k Fisher-Yates steps that reach
    // [k, n)
    let mut h = 0;
    if 0 < k && k < n {
        h = k;
        let q = f64::from(k) / f64::from(n);
        let logq = (-q).ln_1p();
        let mut i = 0;
        loop {
            let g = (1.0 - rng.random::<f64>()).ln() / logq;
            if g >= f64::from(k - i) {
                break;
            }
            i += g as u32;
            if rng.random::<f64>() * q < f64::from(k - i) / f64::from(n - i) {
                h -= 1;
            }
            i += 1;
            if i >= k {
                break;
            }
        }
    }
    // Step 2: h draws with replacement from [k, n), largest first
    let mut low = k - h;
    let range = n - k;
    let mut prev = range;
    let mut a = 1.0;
    let mut out = res.iter_mut();
    while h > 0 {
        a *= rng.random::<f64>().powf(1.0 / f64::from(h));
        let s = (a * f64::from(range)) as u32;
        if s < prev {
            *out.next().unwrap() = n - 1 - (k + s);
            prev = s;
        } else {
            low += 1;
        }
        h -= 1;
    }
    // Step 3: choose low of the k positions in [0, k), largest first, by
    // Vitter's Algorithm A
    let mut m = k;
    while low > 0 {
        let u: f64 = rng.random();
        let mut s = 0;
        let mut f = f64::from(low) / f64::from(m);
        while f < u && s < m - low {
            f = 1.0 - (1.0 - f64::from(low) / f64::from(m - s - 1)) * (1.0 - f);
            s += 1;
        }
        m -= s + 1;
        low -= 1;
        *out.next().unwrap() = n - 1 - m;
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    hiddenshuffle(rng, length, res);
}

pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    hiddenshuffle(rng, length, res);
    for i in 0..res.len() as u32 {
        let r = rng.random_range(0..=i);
        res.swap(i as usize, r as usize);
    }
}
