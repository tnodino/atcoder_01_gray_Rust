// https://atcoder.jp/contests/abc190/tasks/abc190_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        D: usize,
    }
    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
        }
        if X < S && Y > D {
            println!("Yes");
            return;
        }
    }
    println!("No");
}