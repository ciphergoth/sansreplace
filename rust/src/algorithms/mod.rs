mod cardchoose;
mod floyd_f2;
mod quadratic_f2;
mod quadratic_reject;
mod reject;

pub fn algorithms<R>() -> Vec<(&'static str, Box<dyn Fn(&mut R, u32, &mut [u32])>)>
where
    R: rand::Rng + ?Sized + 'static,
{
    vec![
        ("cardchoose", Box::new(cardchoose::random_order)),
        ("floyd_f2", Box::new(floyd_f2::random_order)),
        ("quadratic_f2", Box::new(quadratic_f2::random_order)),
        ("quadratic_reject", Box::new(quadratic_reject::random_order)),
        ("reject", Box::new(reject::random_order)),
    ]
}
