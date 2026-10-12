pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len();
    let mut done = std::collections::HashSet::with_capacity(amount as usize);
    for i in 0..amount {
        loop {
            let t = rng.random_range(0..length);
            if done.insert(t) {
                res[i] = t;
                break;
            }
        }
    }
}
