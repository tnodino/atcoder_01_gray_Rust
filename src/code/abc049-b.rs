// https://atcoder.jp/contests/abc049/tasks/abc049_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        _W: usize,
    }
    for _ in 0..H {
        input! {
            C: String,
        }
        for _ in 0..2 {
            println!("{}", C);
        }
    }
}