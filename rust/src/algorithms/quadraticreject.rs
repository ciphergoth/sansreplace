pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len();
    for i in 0..amount {
        loop {
            let t = rng.random_range(0..length);
            if !res[..i].contains(&t) {
                res[i] = t;
                break;
            }
        }
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    random_order(rng, length, res);
    res.sort_unstable();
}
