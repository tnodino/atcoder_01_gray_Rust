// https://atcoder.jp/contests/abc099/tasks/abc099_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if (N - 1) / 999 == 0 {
        println!("ABC");
    }
    else {
        println!("ABD");
    }
}