// https://atcoder.jp/contests/abc092/tasks/abc092_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
        mut X: usize,
    }
    for _ in 0..N {
        input! {
            A: usize,
        }
        X += (D - 1) / A + 1;
    }
    println!("{}", X);
}