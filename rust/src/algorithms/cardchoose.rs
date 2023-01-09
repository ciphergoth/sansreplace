pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::Rng + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let t = length - amount + 1;
    for i in 0..amount {
        let r = rng.gen_range(0..t + i);
        if r < t {
            res[i as usize] = r;
        } else {
            res[i as usize] = res[(r - t) as usize];
        }
    }
    res.sort();
    for i in 0..amount {
        let r = rng.gen_range(0..=i);
        let t = res[i as usize] + i;
        res[i as usize] = res[r as usize];
        res[r as usize] = t;
    }
}
