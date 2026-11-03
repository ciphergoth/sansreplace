//! The Rust implementations, as a library that python/load_rust.py loads and
//! times as python/load_cpp.py does the C++ ones: algorithm looks one up by
//! its order and name, and timefunc_s and call take what it returns, as
//! cpp/timing.cpp's timefunc_s takes a function pointer.

use std::{cell::RefCell, ffi::CStr, ffi::c_char, time::Instant};

use rand::rngs::SmallRng;

mod algorithms;

type Rng = SmallRng;
type Algorithm = algorithms::Algorithm<Rng>;

thread_local! {
    static RNG: RefCell<Rng> = RefCell::new(rand::make_rng::<Rng>());
}

/// The algorithm with this name for this order, which is "random", "sorted"
/// or, for the one that does nothing, "donothing". It lives as long as the
/// library.
///
/// # Safety
///
/// Both must be NUL-terminated strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn algorithm(sorttype: *const c_char, name: *const c_char) -> *const Algorithm {
    let sorttype = unsafe { CStr::from_ptr(sorttype) }.to_str().unwrap();
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    let mut table = match sorttype {
        "donothing" => vec![(
            "donothing",
            Box::new(|_: &mut Rng, _: u32, _: &mut [u32]| {}) as Algorithm,
        )],
        "random" => algorithms::algorithms(algorithms::SortType::Random),
        "sorted" => algorithms::algorithms(algorithms::SortType::Sorted),
        _ => panic!("unknown order {sorttype}"),
    };
    let i = table
        .iter()
        .position(|(n, _)| *n == name)
        .unwrap_or_else(|| panic!("unknown algorithm {name}"));
    Box::leak(Box::new(table.swap_remove(i).1))
}

/// One call, writing k numbers to result
///
/// # Safety
///
/// f must come from algorithm, and result must point to k writable u32s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call(f: *const Algorithm, n: u32, k: u32, result: *mut u32) {
    let f = unsafe { &*f };
    let result = unsafe { std::slice::from_raw_parts_mut(result, k as usize) };
    RNG.with_borrow_mut(|rng| f(rng, n, result));
}

/// Total time for iters calls, like timefunc_s in cpp/timing.cpp
///
/// # Safety
///
/// f must come from algorithm.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timefunc_s(f: *const Algorithm, n: u32, k: u32, iters: u64) -> f64 {
    let f = unsafe { &*f };
    RNG.with_borrow_mut(|rng| {
        // Start the output on a cache line, since malloc only promises 16
        // bytes, and the quadratic algorithms scan it on every draw: how many
        // of their loads and stores straddle two cache lines would otherwise
        // depend on where malloc happened to put it. Vec can't ask for more
        // alignment, so allocate 15 spare elements and start at the first one
        // on a 64-byte boundary.
        let mut buf = vec![0; k as usize + 15];
        let offset = buf.as_ptr().align_offset(64);
        let result = &mut buf[offset..offset + k as usize];
        let start = Instant::now();
        for _ in 0..iters {
            f(rng, n, result);
            std::hint::black_box(&result);
        }
        start.elapsed().as_secs_f64()
    })
}
