// https://atcoder.jp/contests/arc023/tasks/arc023_1

use proconio::input;
use proconio::fastout;

fn f(mut y: usize, mut m: usize, d: usize) -> usize {
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    return y * 365 + y / 4 - y / 100 + y / 400 + (m + 1) * 306 / 10 + d - 429;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (y, m, d): (usize, usize, usize),
    }
    println!("{}", f(2014, 5, 17) - f(y, m, d));
}