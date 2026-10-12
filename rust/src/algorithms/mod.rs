mod cardchoose;
mod floydf2;
mod hsel;
mod iterativechoose;
mod quadraticf2;
mod quadraticreject;
mod rejectionsample;
mod reservoirsample;
mod select;

pub type Algorithm<R> = Box<dyn Fn(&mut R, u32, &mut [u32])>;

pub fn algorithms<R>() -> Vec<(&'static str, Algorithm<R>)>
where
    R: rand::RngExt + ?Sized + 'static,
{
    vec![
        ("cardchoose", Box::new(cardchoose::random_order)),
        ("floydf2", Box::new(floydf2::random_order)),
        ("hsel", Box::new(hsel::random_order)),
        ("iterativechoose", Box::new(iterativechoose::random_order)),
        ("quadraticf2", Box::new(quadraticf2::random_order)),
        ("quadraticreject", Box::new(quadraticreject::random_order)),
        ("rejectionsample", Box::new(rejectionsample::random_order)),
        ("reservoirsample", Box::new(reservoirsample::random_order)),
        ("select", Box::new(select::random_order)),
    ]
}
