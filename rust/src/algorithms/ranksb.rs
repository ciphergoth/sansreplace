//! RANKSB, from Nijenhuis and Wilf, "Combinatorial Algorithms for Computers
//! and Calculators" (2nd edition, 1978), chapter 4, following the book's
//! program, which uses only the k words of the output. See
//! python/algorithms/ranksb.py for an overview; this uses the same bins, the
//! same test for accepting a draw, and the same steps, but keeps each bin's
//! count in the output array.
//!
//! We depart from the book's program in two ways. It indexes slots, values and
//! bins from 1, so that 0 can mark an empty slot; we index from 0. And after
//! compacting the counts, its step (D) marks out each bin's slots, writing the
//! bin number into the rightmost, and then steps (E)-(H) scan for those marks
//! to fill the bins. We fill each bin's slots as soon as we know where they
//! are, which needs no marks.

fn ranksb<R>(rng: &mut R, n: u32, a: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    let k = a.len() as u32;
    // Bin l is the values lo(l) <= x < lo(l + 1)
    let lo = |l: u32| (u64::from(l) * u64::from(n) / u64::from(k)) as u32;
    let bin = |x: u32| ((u64::from(x + 1) * u64::from(k) - 1) / u64::from(n)) as usize;
    // (A) a[l] holds lo(l) + the number of values chosen in bin l. Each
    // nonempty bin's entry is then one more than a value in the bin, so it
    // says which bin it counts.
    for (l, v) in (0..).zip(a.iter_mut()) {
        *v = lo(l);
    }
    // (B) Simulate rejection sampling: a draw duplicates a value already
    // chosen in its bin with probability count / (bin size), so reject it if
    // it lands among the first count values of its bin
    let mut c = k;
    while c > 0 {
        let x = rng.random_range(0..n);
        let l = bin(x);
        if x >= a[l] {
            a[l] += 1;
            c -= 1;
        }
    }
    // (C) Move the entries for nonempty bins to the left
    let mut p = 0;
    for l in 0..a.len() {
        if a[l] != lo(l as u32) {
            a[p] = a[l];
            p += 1;
        }
    }
    // An entry says which bin it counts and how many values that bin needs
    #[derive(Default)]
    struct Bin {
        l: u32,
        lo: u32,
        count: usize,
    }
    let decode = |e: u32| {
        let l = bin(e - 1) as u32;
        Bin {
            l,
            lo: lo(l),
            count: (e - lo(l)) as usize,
        }
    };
    // Fill each bin's slots, working from the right. The q nonempty bins
    // before the one a[q] counts each need at least one slot, so its slots
    // are all at q or later, and filling them overwrites no entry we have yet
    // to read. Decoding takes divisions, so we decode each entry before
    // filling the bin to its right, while the CPU has other work to do,
    // rather than leaving the fill to wait for them.
    let mut s = a.len();
    let mut b = if p > 0 {
        decode(a[p - 1])
    } else {
        Bin::default()
    };
    for q in (0..p).rev() {
        let next = if q > 0 {
            decode(a[q - 1])
        } else {
            Bin::default()
        };
        // m values in the bin are not yet chosen
        let mut m = lo(b.l + 1) - b.lo;
        // Fill the slots s - count to s - 1, from the right, with distinct
        // values from the bin in increasing order
        for r in (s - b.count..s).rev() {
            // (F) Choose one of the m values not yet chosen
            let mut x = b.lo + rng.random_range(0..m);
            // (G) Skip over the values already chosen, shifting them left
            let mut i = r + 1;
            while i < s && x >= a[i] {
                x += 1;
                a[i - 1] = a[i];
                i += 1;
            }
            // (H) Insert x
            a[i - 1] = x;
            m -= 1;
        }
        s -= b.count;
        b = next;
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
