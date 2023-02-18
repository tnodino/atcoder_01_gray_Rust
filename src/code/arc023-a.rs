// https://atcoder.jp/contests/arc023/tasks/arc023_1

use proconio::input;
use proconio::fastout;

fn elapsed_days(y: usize, m : usize, d: usize) -> usize {
    return (365 * y) + (y / 4) - (y / 100) + (y / 400) + (306 * (m + 1) / 10) + d - 429;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut y: usize,
        mut m: usize,
        d: usize,
    }
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let A = elapsed_days(y, m, d);
    let B = elapsed_days(2014, 5, 17);
    println!("{}", B - A);
}