// https://atcoder.jp/contests/abc174/tasks/abc174_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
    }
    if X >= 30 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}