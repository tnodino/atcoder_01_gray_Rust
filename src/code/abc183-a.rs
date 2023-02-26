// https://atcoder.jp/contests/abc183/tasks/abc183_a

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn ReLU(x: isize) -> isize {
    if x >= 0 {
        return x;
    }
    return 0;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: isize,
    }
    println!("{}", ReLU(x));
}