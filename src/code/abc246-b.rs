// https://atcoder.jp/contests/abc246/tasks/abc246_b

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
    }
    let dist = hypot(A, B);
    println!("{} {}", A / dist, B / dist);
}