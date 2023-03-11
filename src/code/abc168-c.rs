// https://atcoder.jp/contests/abc168/tasks/abc168_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
        H: f64,
        M: f64,
    }
    let rad = (360. * (H / 12. + M / 60. / 12. - M / 60.)).to_radians();
    let rsq = A * A + B * B - 2. * A * B * rad.cos();
    println!("{}", rsq.sqrt());
}