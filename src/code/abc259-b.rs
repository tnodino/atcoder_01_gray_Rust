// https://atcoder.jp/contests/abc259/submissions/33129767

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }
    let d = d.to_radians();
    let x = [[d.cos(), -d.sin()], [d.sin(), d.cos()]];
    let y = [[a], [b]];
    let mut ans = [[0.], [0.]];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..1 {
                ans[i][k] += x[i][j] * y[j][k];
            }
        }
    }
    println!("{} {}", ans[0][0], ans[1][0]);
}