pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut ix = 0;
    for i in 0..length {
        let r = rng.random_range(0..length - i);
        if r < amount - ix {
            res[ix as usize] = i;
            ix += 1;
        }
    }
}

pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut ix = 0;
    for i in 0..length {
        let r = rng.random_range(0..length - i);
        if r < amount - ix {
            let rr = rng.random_range(0..=ix) as usize;
            res[ix as usize] = res[rr];
            res[rr] = i;
            ix += 1;
        }
    }
}
