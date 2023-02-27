// https://atcoder.jp/contests/abc028/tasks/abc028_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", match N {
        v if v < 60 => "Bad",
        v if v < 90 => "Good",
        v if v < 100 => "Great",
        _ => "Perfect",
    });
}