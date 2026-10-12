pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let mut done = std::collections::HashSet::with_capacity(res.len());
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
