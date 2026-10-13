fn cardchoose<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    // t would overflow for length = u32::MAX and amount = 0
    if amount == 0 {
        return;
    }
    let t = length - amount + 1;
    for i in 0..amount {
        let r = rng.random_range(0..t + i);
        if r < t {
            res[i as usize] = r;
        } else {
            res[i as usize] = res[(r - t) as usize];
        }
    }
    res.sort_unstable();
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    cardchoose(rng, length, res);
    for (r, i) in res.iter_mut().zip(0..) {
        *r += i;
    }
}

pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    cardchoose(rng, length, res);
    for i in 0..res.len() as u32 {
        let r = rng.random_range(0..=i);
        let t = res[i as usize] + i;
        res[i as usize] = res[r as usize];
        res[r as usize] = t;
    }
}
