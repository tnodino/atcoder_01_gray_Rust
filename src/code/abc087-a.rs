// https://atcoder.jp/contests/abc087/tasks/abc087_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize,
        A: usize,
        B: usize,
    }
    X -= A;
    println!("{}", X - X / B * B);
}