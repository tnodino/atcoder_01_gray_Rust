// https://atcoder.jp/contests/zone2021/tasks/zone2021_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: f64,
        H: f64,
    }
    let mut ans = 0.;
    for _ in 0..N {
        input! {
            d: f64,
            h: f64,
        }
        let high = h - d * (H - h) / (D - d);
        if ans < high {
            ans = high;
        }
    }
    println!("{}", ans);
}