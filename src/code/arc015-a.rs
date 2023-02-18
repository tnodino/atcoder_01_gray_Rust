// https://atcoder.jp/contests/arc015/tasks/arc015_1

use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: f64,
    }
    println!("{}", (9. / 5. * n) + 32.);
}