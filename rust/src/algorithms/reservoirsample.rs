pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    for i in 0..amount {
        let r = rng.random_range(0..=i) as usize;
        res[i as usize] = res[r];
        res[r] = i;
    }
    for i in amount..length {
        let r = rng.random_range(0..=i);
        if r < amount {
            res[r as usize] = i;
        }
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    for (r, i) in res.iter_mut().zip(0..) {
        *r = i;
    }
    for i in amount..length {
        let r = rng.random_range(0..=i);
        if r < amount {
            res[r as usize] = i;
        }
    }
    res.sort_unstable();
}
