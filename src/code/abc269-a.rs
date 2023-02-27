// https://atcoder.jp/contests/abc269/tasks/abc269_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    println!("{}", (a + b) * (c - d));
    println!("Takahashi");
}