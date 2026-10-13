//! Vitter's Algorithm D, from "An Efficient Algorithm for Sequential Random
//! Sampling" (ACM TOMS 13(1), 1987), following the implementation in its
//! appendix. See python/algorithms/vitterd.py for an overview.

const ALPHA_INV: u32 = 13;

/// Algorithm A: write the `res.len()` values chosen from the `n` values
/// starting at `j`
fn vittera<R>(rng: &mut R, mut n: u32, mut j: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    let mut k = res.len() as u32;
    let mut top = n - k;
    let mut out = res.iter_mut();
    while k >= 2 {
        let v: f64 = rng.random();
        let mut quot = f64::from(top) / f64::from(n);
        while quot > v {
            j += 1;
            top -= 1;
            n -= 1;
            quot *= f64::from(top) / f64::from(n);
        }
        *out.next().unwrap() = j;
        j += 1;
        n -= 1;
        k -= 1;
    }
    if k == 1 {
        *out.next().unwrap() = j + rng.random_range(0..n);
    }
}

fn vitterd<R>(rng: &mut R, mut n: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    let mut k = res.len() as u32;
    if k == 0 {
        return;
    }
    let mut j: u32 = 0;
    let mut ninv = 1.0 / f64::from(k);
    let mut vprime = rng.random::<f64>().powf(ninv);
    let mut qu1 = n - k + 1;
    let mut threshold = u64::from(ALPHA_INV) * u64::from(k);
    let mut ix = 0;
    while k > 1 && threshold < u64::from(n) {
        let nmin1inv = 1.0 / f64::from(k - 1);
        let nreal = f64::from(n);
        let s;
        loop {
            // Step D2: generate U and X
            let mut x;
            let mut sx;
            loop {
                x = nreal * (1.0 - vprime);
                sx = x as u32;
                if sx < qu1 {
                    break;
                }
                vprime = rng.random::<f64>().powf(ninv);
            }
            let u: f64 = rng.random();
            // Step D3: accept?
            let y1 = (u * nreal / f64::from(qu1)).powf(nmin1inv);
            vprime = y1 * (1.0 - x / nreal) * (f64::from(qu1) / f64::from(qu1 - sx));
            if vprime <= 1.0 {
                s = sx;
                break;
            }
            // Step D4: accept?
            let mut y2 = 1.0;
            let mut top = nreal - 1.0;
            let (mut bottom, limit) = if k - 1 > sx {
                (f64::from(n - k), n - sx)
            } else {
                (f64::from(n - sx - 1), qu1)
            };
            for _ in limit..n {
                y2 *= top / bottom;
                top -= 1.0;
                bottom -= 1.0;
            }
            if nreal / (nreal - x) >= y1 * y2.powf(nmin1inv) {
                vprime = rng.random::<f64>().powf(nmin1inv);
                s = sx;
                break;
            }
            vprime = rng.random::<f64>().powf(ninv);
        }
        // Step D5: select the (S + 1)st value
        j += s;
        res[ix] = j;
        ix += 1;
        j += 1;
        n -= s + 1;
        k -= 1;
        ninv = nmin1inv;
        qu1 -= s;
        threshold -= u64::from(ALPHA_INV);
    }
    if k > 1 {
        vittera(rng, n, j, &mut res[ix..]);
    } else {
        // The appendix reuses vprime here, but it can round to exactly 1,
        // which would give j + n
        res[ix] = j + rng.random_range(0..n);
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    vitterd(rng, length, res);
}

pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    vitterd(rng, length, res);
    for i in 0..res.len() as u32 {
        let r = rng.random_range(0..=i);
        res.swap(i as usize, r as usize);
    }
}
