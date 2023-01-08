use rand::Rng;

pub fn random_order<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: Rng + ?Sized,
{
    let amount = res.len();
    let t = length - amount + 1;
    for i in 0..amount {
        let r = rng.gen_range(0..t + i);
        if r < t {
            res[i] = r;
        } else {
            res[i] = res[r - t];
        }
    }
    res.sort();
    for i in 0..amount {
        let r = rng.gen_range(0..=i);
        let t = res[i] + i;
        res[i] = res[r];
        res[r] = t;
    }
}
