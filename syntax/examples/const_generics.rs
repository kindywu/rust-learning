use std::ops::Mul;

fn main() {
    println!("{}", dot_product([1, 2, 3, 4], [1, 2, 3, 4]));
    println!(
        "{}",
        dot_product([1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0])
    );
}

fn dot_product<const N: usize, T>(a: [T; N], b: [T; N]) -> T
where
    T: Default + Mul<Output = T> + Copy,
{
    let mut sum = Default::default();
    for i in 0..N {
        sum = a[i] * b[i];
    }
    sum
}
