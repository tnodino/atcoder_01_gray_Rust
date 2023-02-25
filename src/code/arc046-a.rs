// https://atcoder.jp/contests/arc046/tasks/arc046_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for _ in 0..(N-1)/9+1 {
        print!("{}", (N - 1) % 9 + 1);
    }
    println!();
}