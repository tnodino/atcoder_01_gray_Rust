// https://atcoder.jp/contests/abc121/tasks/abc121_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        h: usize,
        w: usize,
    }
    println!("{}", H * W + h * w - h * W - H * w );
}