// https://atcoder.jp/contests/abc262/tasks/abc262_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut Y: usize,
    }
    while Y % 4 != 2 {
        Y += 1;
    }
    println!("{}", Y);
}