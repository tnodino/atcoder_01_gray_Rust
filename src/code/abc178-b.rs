// https://atcoder.jp/contests/abc178/tasks/abc178_b

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
    let array = [a * c, a * d, b * c, b * d];
    println!("{}", array.iter().max().unwrap());
}