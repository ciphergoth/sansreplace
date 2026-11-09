//! Parts of the algorithms, timed on their own to compare the two standard
//! libraries, as in cpp/components.cpp: fill draws k random numbers below n
//! into the output, and each of the others does the same and then one thing
//! more, as the algorithms do it, so that its time less fill's is what that
//! thing costs on k random numbers.

use std::collections::HashSet;

use crate::algorithms::Algorithm;

fn fill<R>(rng: &mut R, length: u32, res: &mut [u32])
where
    R: rand::RngExt + ?Sized,
{
    for r in res.iter_mut() {
        *r = rng.random_range(0..length);
    }
}

pub fn components<R>() -> Vec<(&'static str, Algorithm<R>)>
where
    R: rand::RngExt + ?Sized + 'static,
{
    vec![
        ("fill", Box::new(|rng: &mut R, length: u32, res: &mut [u32]| fill(rng, length, res))),
        (
            "fillsort",
            Box::new(|rng: &mut R, length: u32, res: &mut [u32]| {
                fill(rng, length, res);
                res.sort_unstable();
            }),
        ),
        // The shuffle that puts the sorted algorithms' output in random order
        (
            "fillshuffle",
            Box::new(|rng: &mut R, length: u32, res: &mut [u32]| {
                fill(rng, length, res);
                for i in 0..res.len() as u32 {
                    let r = rng.random_range(0..i + 1);
                    res.swap(i as usize, r as usize);
                }
            }),
        ),
        // The set set-based rejection sampling keeps. Its size goes into the
        // output, so that building it can't be optimised away.
        (
            "hashinsert",
            Box::new(|rng: &mut R, length: u32, res: &mut [u32]| {
                fill(rng, length, res);
                let mut done = HashSet::with_capacity(res.len());
                for &r in res.iter() {
                    done.insert(r);
                }
                res[0] = done.len() as u32;
            }),
        ),
    ]
}
