// https://atcoder.jp/contests/abc196/tasks/abc196_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _a: isize,
        b: isize,
        c: isize,
        _d: isize,
    }
    println!("{}", b - c);
}