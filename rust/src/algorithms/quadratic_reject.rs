pub fn random_order<R>(rng: &mut R, length: usize, res: &mut [usize])
where
    R: rand::Rng + ?Sized,
{
    let amount = res.len();
    for i in 0..amount {
        loop {
            let t = rng.gen_range(0..length);
            if !res[..i].contains(&t) {
                res[i] = t;
                break;
            }
        }
    }
}
