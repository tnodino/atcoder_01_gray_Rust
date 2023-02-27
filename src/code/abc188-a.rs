// https://atcoder.jp/contests/abc188/tasks/abc188_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
        Y: isize,
    }
    if (X - Y).abs() < 3 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}