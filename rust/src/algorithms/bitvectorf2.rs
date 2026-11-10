//! Floyd's F2, as in floydf2.rs, with a bit vector of length bits as its set:
//! each step tests and sets one bit, and sorted output comes from scanning
//! the bit vector, with no sort.

/// Call f with a cleared bit vector of length bits. It starts on a cache line, as
/// SELECT's array does: Vec can't ask for more alignment, so allocate 7 spare
/// words and start at the first one on a 64-byte boundary.
fn with_bit_vector<T>(length: u32, f: impl FnOnce(&mut [u64]) -> T) -> T {
    let words = (length as usize).div_ceil(64);
    let mut buf: Vec<u64> = Vec::with_capacity(words + 7);
    let start = buf.as_ptr().align_offset(64);
    buf.resize(start + words, 0);
    f(&mut buf[start..])
}

/// Set bit x, and say whether it was already set
fn test_and_set(bits: &mut [u64], x: u32) -> bool {
    let word = &mut bits[(x >> 6) as usize];
    let mask = 1u64 << (x & 63);
    let was = *word & mask != 0;
    *word |= mask;
    was
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    with_bit_vector(length, |bits| {
        for i in 0..amount {
            let j = length - amount + i;
            let t = rng.random_range(0..j + 1);
            if test_and_set(bits, t) {
                test_and_set(bits, j);
            }
        }
        let mut out = res.iter_mut();
        for (w, &word) in (0u32..).zip(bits.iter()) {
            let mut b = word;
            while b != 0 {
                *out.next().unwrap() = w * 64 + b.trailing_zeros();
                b &= b - 1;
            }
        }
    });
}

/// As floydf2::random_order: on a collision, j goes in a random earlier
/// place, and what was there goes at the end
pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    with_bit_vector(length, |bits| {
        for i in 0..amount {
            let j = length - amount + i;
            let t = rng.random_range(0..j + 1);
            if !test_and_set(bits, t) {
                res[i as usize] = t;
            } else {
                test_and_set(bits, j);
                let ix = rng.random_range(0..i);
                res[i as usize] = res[ix as usize];
                res[ix as usize] = j;
            }
        }
    });
}
