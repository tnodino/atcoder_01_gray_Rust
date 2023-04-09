// https://atcoder.jp/contests/abc259/tasks/abc259_b

use proconio::input;
use proconio::fastout;
use libm::{hypot, atan2, cos, sin};
use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }
    let r = hypot(a, b);
    let mut theta = atan2(b, a);
    theta += d * PI / 180.;
    println!("{} {}", cos(theta) * r, sin(theta) * r);
}