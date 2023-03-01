// https://atcoder.jp/contests/abc273/tasks/abc273_a

use proconio::input;
use proconio::fastout;

fn f(x: usize) -> usize {
    if x == 0 {
        return 1;
    }
    return x * f(x-1);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", f(N));
}