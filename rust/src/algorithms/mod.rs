mod cardchoose;
mod floydf2;
mod hsel;
mod iterativechoose;
mod quadraticf2;
mod quadraticreject;
mod rejectionsample;
mod reservoirsample;
mod select;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum SortType {
    Random,
    Sorted,
}

pub type Algorithm<R> = Box<dyn Fn(&mut R, u32, &mut [u32])>;

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
    algorithms![
        cardchoose,
        floydf2,
        hsel,
        iterativechoose,
        quadraticf2,
        quadraticreject,
        rejectionsample,
        reservoirsample,
        select
    ]
}
