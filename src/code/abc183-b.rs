// https://atcoder.jp/contests/abc183/tasks/abc183_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Sx: f64,
        Sy: f64,
        Gx: f64,
        Gy: f64,
    }
    println!("{}", (Sx * Gy + Gx * Sy) / (Sy + Gy));
}