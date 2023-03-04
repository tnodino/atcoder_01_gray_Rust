// https://atcoder.jp/contests/abc141/tasks/abc141_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", match S.as_str() {
        "Sunny" => "Cloudy",
        "Cloudy" => "Rainy",
        _ => "Sunny",
    });
}