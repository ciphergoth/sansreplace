pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.random_range(0..j + 1);
        // The entries are distinct, so at most one matches; writing every
        // entry without stopping lets the compiler vectorise this loop.
        for x in &mut res[..(i as usize)] {
            *x = if *x == t { j } else { *x };
        }
        res[i as usize] = t;
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.random_range(0..j + 1);
        // Before sorting only the set matters, so where t is already present,
        // add j instead of moving the old t to j: the set is the same, and the
        // scan only reads.
        res[i as usize] = if res[..(i as usize)].contains(&t) { j } else { t };
    }
    res.sort_unstable();
}
