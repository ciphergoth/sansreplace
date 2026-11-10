pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let mut done = super::HashSet::with_capacity_and_hasher(res.len(), Default::default());
    for r in res.iter_mut() {
        loop {
            let t = rng.random_range(0..length);
            if done.insert(t) {
                *r = t;
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
