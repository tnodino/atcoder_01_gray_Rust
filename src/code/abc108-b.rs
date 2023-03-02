// https://atcoder.jp/contests/abc108/tasks/abc108_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }
    let x = x2 - x1;
    let y = y2 - y1;
    println!("{} {} {} {}", x2 - y, y2 + x, x1 - y, y1 + x);
}