pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut options = std::collections::HashMap::with_capacity(2 * amount as usize);
    for i in 0..amount {
        let r = rng.random_range(i..length);
        res[i as usize] = *options.get(&r).unwrap_or(&r);
        let oi = *options.get(&i).unwrap_or(&i);
        options.insert(r, oi);
    }
}
