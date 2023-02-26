// https://atcoder.jp/contests/abc202/tasks/abc202_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    println!("{}", 21 - a - b - c);
}