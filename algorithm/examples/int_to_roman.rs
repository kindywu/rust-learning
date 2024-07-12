use anyhow::{anyhow, Result};

fn main() {
    println!("{:?}", int_to_roman(3));
    println!("{:?}", int_to_roman(6));

    println!("{:?}", int_to_roman(1));
    println!("{:?}", int_to_roman(5));
    println!("{:?}", int_to_roman(10));
    println!("{:?}", int_to_roman(50));
    println!("{:?}", int_to_roman(100));
    println!("{:?}", int_to_roman(500));
    println!("{:?}", int_to_roman(1000));

    println!("{:?}", int_to_roman(1994));
    println!("{:?}", int_to_roman(3999));
}

const VALUE_ARRAY: &[u16] = &[1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
const SYMBOL_ARRAY: &[&str] = &[
    "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
];

fn int_to_roman(mut num: u16) -> Result<String> {
    let mut roman = String::new();
    if num <= 0 || num > 3999 {
        return Err(anyhow!("num {num} is not between 1 and 3999"));
    }

    for i in 0..VALUE_ARRAY.len() {
        let v = VALUE_ARRAY[i];
        while num >= v {
            num -= v;
            roman.push_str(SYMBOL_ARRAY[i]);
        }
    }

    Ok(roman)
}
