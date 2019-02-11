#![feature(plugin)]
#![plugin(roman_numerals)]

fn main() {
    println!("{}", rn!(MMXV));
    assert_eq!(rn!(MMXV), 2015);
}