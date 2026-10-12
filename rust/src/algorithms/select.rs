pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut options: Vec<u32> = (0..length).collect();
    for i in 0..amount {
        let r = rng.random_range(i..length) as usize;
        res[i as usize] = options[r];
        options[r] = options[i as usize];
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    random_order(rng, length, res);
    res.sort_unstable();
}
