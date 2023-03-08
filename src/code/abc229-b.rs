// https://atcoder.jp/contests/abc229/tasks/abc229_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
    }
    while A > 0 && B > 0 {
        let x = A % 10;
        let y = B % 10;
        if x + y >= 10 {
            println!("Hard");
            return;
        }
        A /= 10;
        B /= 10;
    }
    println!("Easy");
}