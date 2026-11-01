pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    assert!(res.len() <= (length as usize));
    let amount = res.len() as u32;
    // Start options on a cache line, since malloc only promises 16 bytes, and
    // how many of the fill's stores straddle two cache lines would otherwise
    // depend on where malloc happened to put it. Vec can't ask for more
    // alignment, so allocate 15 spare elements and start at the first one on
    // a 64-byte boundary.
    let mut buf: Vec<u32> = Vec::with_capacity(length as usize + 15);
    let start = buf.as_ptr().align_offset(64);
    buf.resize(start, 0);
    buf.extend(0..length);
    let options = &mut buf[start..];
    for i in 0..amount {
        let r = rng.random_range(i..length) as usize;
        res[i as usize] = options[r];
        options[r] = options[i as usize];
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    random_order(rng, length, res);
    res.sort_unstable();
}
