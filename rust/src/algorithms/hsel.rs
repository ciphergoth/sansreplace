use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn random_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    // A Fisher-Yates shuffle of the first k entries of a virtual array of
    // [0, length). The first k entries of the virtual array are res itself;
    // the map holds only the entries beyond k that a swap has changed.
    let k = res.len() as u32;
    let mut options: HashMap<u32, u32> = HashMap::with_capacity(k as usize);
    for (i, x) in res.iter_mut().enumerate() {
        *x = i as u32;
    }
    for i in 0..k {
        let r = rng.random_range(i..length);
        if r < k {
            res.swap(i as usize, r as usize);
        } else {
            let res_i = res[i as usize];
            match options.entry(r) {
                Entry::Vacant(e) => {
                    e.insert(res_i);
                    res[i as usize] = r;
                }
                Entry::Occupied(mut e) => {
                    res[i as usize] = std::mem::replace(e.get_mut(), res_i);
                }
            }
        }
    }
}

pub fn sorted_order<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    random_order(rng, length, res);
    res.sort_unstable();
}
