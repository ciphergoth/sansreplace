mod cardchoose;
mod floydf2;
mod hiddenshuffle;
mod hsel;
mod iterativechoose;
mod quadraticf2;
mod quadraticreject;
mod ranksb;
mod rejectionsample;
mod reservoirsample;
mod select;
mod vitterd;

#[derive(Clone, Copy, Debug)]
pub enum SortType {
    Random,
    Sorted,
}

pub type Algorithm<R> = Box<dyn Fn(&mut R, u32, &mut [u32])>;

// The algorithms draw from 0..x + 1 rather than 0..=x. rand's random_range has
// no #[inline], and LLVM kept the RangeInclusive instance out of line once it
// had several callers, so that every draw was a call, passing the generator's
// state through memory: twice as slow as the inlined draw in a tight loop.

pub fn algorithms<R>(sort_type: SortType) -> Vec<(&'static str, Algorithm<R>)>
where
    R: rand::RngExt + ?Sized + 'static,
{
    macro_rules! algorithms {
        ($($name:ident),*) => {
            vec![$((
                stringify!($name),
                match sort_type {
                    SortType::Random => Box::new($name::random_order) as Algorithm<R>,
                    SortType::Sorted => Box::new($name::sorted_order),
                },
            )),*]
        };
    }
    // Same order as python/algorithms/__init__.py
    algorithms![
        quadraticreject,
        rejectionsample,
        ranksb,
        floydf2,
        quadraticf2,
        iterativechoose,
        reservoirsample,
        vitterd,
        hiddenshuffle,
        select,
        hsel,
        cardchoose
    ]
}
