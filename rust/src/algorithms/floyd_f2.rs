use rand::Rng;

pub fn random_order<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: Rng + ?Sized,
{
    let amount = res.len();
    let mut done = std::collections::HashSet::with_capacity(amount);
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.gen_range(0..=j);
        if done.insert(t) {
            res[i] = t;
        } else {
            let ix = rng.gen_range(0..i);
            res[i] = res[ix];
            res[ix] = j;
            done.insert(j);
        }
    }
}
