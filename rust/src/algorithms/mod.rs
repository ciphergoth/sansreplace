mod cardchoose;
mod floyd_f2;
mod quadratic_f2;
mod quadratic_reject;
mod reject;

pub fn algorithms() -> Vec<(
    &'static str,
    Box<dyn Fn(&mut rand::rngs::SmallRng, usize, &mut [usize])>,
)> {
    vec![
        ("cardchoose", Box::new(cardchoose::random_order)),
        ("floyd_f2", Box::new(floyd_f2::random_order)),
        ("reject", Box::new(reject::random_order)),
        ("quadratic_f2", Box::new(quadratic_f2::random_order)),
        ("quadratic_reject", Box::new(quadratic_reject::random_order)),
    ]
}
