// https://atcoder.jp/contests/arc031/tasks/arc031_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Name: String,
    }
    println!("{}", match Name == Name.chars().rev().collect::<String>() {
        true => "YES",
        false => "NO",
    });
}