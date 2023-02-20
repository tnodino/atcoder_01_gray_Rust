// https://atcoder.jp/contests/donuts-2015/tasks/donuts_2015_1

use proconio::input;
use proconio::fastout;
use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: f64,
        D: f64,
    }
    println!("{}", 2. * PI * PI * R * R * D);
}