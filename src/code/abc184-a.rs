// https://atcoder.jp/contests/abc184/tasks/abc184_a

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
    println!("{}", a * d - b * c);
}