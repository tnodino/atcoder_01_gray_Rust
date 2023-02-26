// https://atcoder.jp/contests/abc211/tasks/abc211_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
    }
    println!("{}", (A - B) / 3. + B);
}