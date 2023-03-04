// https://atcoder.jp/contests/abc127/tasks/abc127_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        r: usize,
        D: usize,
        mut x: usize,
    }
    for _ in 0..10 {
        let y = r * x - D;
        println!("{}", y);
        x = y;
    }
}