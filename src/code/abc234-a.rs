// https://atcoder.jp/contests/abc234/tasks/abc234_a

use proconio::input;
use proconio::fastout;

fn f(x: usize) -> usize {
    return x * x + 2 * x + 3;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        t: usize,
    }
    println!("{}", f(f(f(t) + t) + f(f(t))));
}