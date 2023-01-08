use rand::Rng;

pub fn random_order<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: Rng + ?Sized,
{
    let amount = res.len();
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.gen_range(0..=j);
        if let Some(pos) = res[..i].iter().position(|&x| x == t) {
            res[pos] = j;
        }
        res[i] = t;
    }
}
