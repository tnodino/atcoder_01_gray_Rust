// https://atcoder.jp/contests/abc096/tasks/abc096_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a <= b {
        println!("{}", a);
    }
    else {
        println!("{}", a - 1);
    }
}