// https://atcoder.jp/contests/abc231/tasks/abc231_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: f64,
    }
    println!("{}", D / 100.);
}