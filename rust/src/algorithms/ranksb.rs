//! RANKSB, from Nijenhuis and Wilf, "Combinatorial Algorithms for Computers
//! and Calculators" (2nd edition, 1978), chapter 4, following the book's
//! program, which uses only the k words of the output. See
//! python/algorithms/ranksb.py for an overview. The book indexes from 1 and
//! uses 0 to mark an empty slot, so we do the same internally, with `a[i - 1]`
//! for the book's a_i, and subtract 1 from each value at the end.

fn ranksb<R>(rng: &mut R, n: u32, a: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    let k = a.len() as u32;
    if k == 0 {
        return;
    }
    // Bin l is the values zero(l) < x <= zero(l + 1)
    let zero = |l: u64| ((l - 1) * u64::from(n) / u64::from(k)) as u32;
    let bin = |x: u32| (1 + (u64::from(x) * u64::from(k) - 1) / u64::from(n)) as u32;
    let ix = |i: u32| (i - 1) as usize;
    // (A) a_l starts at the "zero point" of bin l
    for l in 1..=k {
        a[ix(l)] = zero(l.into());
    }
    // (B) Choose how many values fall in each bin: a_l - zero(l) counts them
    let mut c = k;
    while c > 0 {
        let x = 1 + rng.random_range(0..n);
        let l = bin(x);
        if x > a[ix(l)] {
            a[ix(l)] += 1;
            c -= 1;
        }
    }
    // (C) Move the entries for nonempty bins to the left
    let mut p = 0;
    for i in 1..=k {
        let m = a[ix(i)];
        a[ix(i)] = 0;
        if m != zero(i.into()) {
            p += 1;
            a[ix(p)] = m;
        }
    }
    // (D) Reserve space for each bin's values, from the right, marking the
    // rightmost slot of each with the bin number
    let mut s = k;
    while p > 0 {
        let l = bin(a[ix(p)]);
        let ds = a[ix(p)] - zero(l.into());
        a[ix(p)] = 0;
        a[ix(s)] = l;
        s -= ds;
        p -= 1;
    }
    // (E)-(H) Fill each bin's slots, from the right, with distinct values
    // from that bin in increasing order
    let mut l = k;
    let (mut r, mut m0, mut m) = (0, 0, 0);
    while l > 0 {
        if a[ix(l)] != 0 {
            // (E) A new bin: its slots end at r, and m values are left in it
            let b = a[ix(l)];
            r = l;
            m0 = 1 + zero(b.into());
            m = zero(u64::from(b) + 1) - m0 + 1;
        }
        // (F) Choose one of the m values not yet chosen
        let mut x = m0 + rng.random_range(0..m);
        // (G) Skip over the values already chosen, shifting them left
        let mut i = l + 1;
        while i <= r && x >= a[ix(i)] {
            x += 1;
            a[ix(i - 1)] = a[ix(i)];
            i += 1;
        }
        // (H) Insert x
        a[ix(i - 1)] = x;
        m -= 1;
        l -= 1;
    }
    for v in a.iter_mut() {
        *v -= 1;
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    ranksb(rng, length, res);
}

pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    ranksb(rng, length, res);
    for i in 0..res.len() as u32 {
        let r = rng.random_range(0..=i);
        res.swap(i as usize, r as usize);
    }
}
