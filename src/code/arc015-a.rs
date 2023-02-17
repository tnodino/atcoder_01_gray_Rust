// https://atcoder.jp/contests/arc015/tasks/arc015_1

use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: f64,
    }
    println!("{}", (9.0 / 5.0 * n) + 32.0);
}