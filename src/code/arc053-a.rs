// https://atcoder.jp/contests/arc053/tasks/arc053_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    println!("{}", H * (W - 1) + (H - 1) * W);
}