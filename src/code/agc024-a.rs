// https://atcoder.jp/contests/agc024/tasks/agc024_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        _C: isize,
        K: usize,
    }
    if K % 2 == 0 {
        println!("{}", A - B);
    }
    else {
        println!("{}", B - A);
    }
}