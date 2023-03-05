// https://atcoder.jp/contests/abc239/tasks/abc239_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: f64,
    }
    println!("{}", (H * (12_800_000. + H)).sqrt());
}