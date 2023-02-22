// https://atcoder.jp/contests/arc034/tasks/arc034_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0.;
    for _ in 0..N {
        input! {
            a: f64,
            b: f64,
            c: f64,
            d: f64,
            e: f64,
        }
        let point = a + b + c + d + e * 110. / 900.;
        if ans < point {
            ans = point;
        }
    }
    println!("{}", ans);
}