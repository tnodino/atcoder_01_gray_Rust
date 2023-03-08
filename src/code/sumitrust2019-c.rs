// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut l = 0;
    let mut r = 0;
    while l < X {
        l += 100;
        r += 105;
        if l <= X && X <= r {
            println!("{}", 1);
            return;
        }
    }
    println!("{}", 0);
}