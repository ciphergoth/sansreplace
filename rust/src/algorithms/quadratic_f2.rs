pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::Rng + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    for i in 0..amount {
        let j = length - amount + i;
        let t = rng.gen_range(0..=j);
        if let Some(pos) = res[..(i as usize)].iter().position(|&x| x == t) {
            res[pos] = j;
        }
        res[i as usize] = t;
    }
}
