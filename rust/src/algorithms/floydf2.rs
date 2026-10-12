pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::Rng + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    let mut done = std::collections::HashSet::with_capacity(amount as usize);
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.gen_range(0..=j);
        if done.insert(t) {
            res[i as usize] = t;
        } else {
            let ix = rng.gen_range(0..i);
            res[i as usize] = res[ix as usize];
            res[ix as usize] = j;
            done.insert(j);
        }
    }
}
