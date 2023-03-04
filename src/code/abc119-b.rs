// https://atcoder.jp/contests/abc119/tasks/abc119_b

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
            x: f64,
            y: String,
        }
        ans += match y.as_str() {
            "JPY" => x,
            _ => 380000. * x,
        };
    }
    println!("{}", ans);
}