// https://atcoder.jp/contests/donuts-2015/tasks/donuts_2015_1

use proconio::input;
use proconio::fastout;
use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (R, D): (f64, f64),
    }
    println!("{}", R * R * PI * D * 2. * PI);
}