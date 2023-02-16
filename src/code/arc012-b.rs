// https://atcoder.jp/contests/arc012/tasks/arc012_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        va: f64,
        vb: f64,
        mut L: f64,
    }
    for _ in 0..N {
        let T = L / va;
        L = T * vb;
    }
    println!("{}", L);
}