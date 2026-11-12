//! Floyd's F2, as in floydf2.rs, with a bit vector of length bits as its set:
//! each step tests and sets one bit, and sorted output comes from scanning
//! the bit vector, with no sort.

// put sets a bit, and says whether it was already set
use fixedbitset::FixedBitSet;

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut bits = FixedBitSet::with_capacity(length as usize);
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.random_range(0..j + 1);
        if bits.put(t as usize) {
            bits.insert(j as usize);
        }
    }
    for (r, x) in res.iter_mut().zip(bits.ones()) {
        *r = x as u32;
    }
}

/// As floydf2::random_order: on a collision, j goes in a random earlier
/// place, and what was there goes at the end
pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut bits = FixedBitSet::with_capacity(length as usize);
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.random_range(0..j + 1);
        if !bits.put(t as usize) {
            res[i as usize] = t;
        } else {
            bits.insert(j as usize);
            let ix = rng.random_range(0..i);
            res[i as usize] = res[ix as usize];
            res[ix as usize] = j;
        }
    }
}
