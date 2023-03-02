// https://atcoder.jp/contests/arc104/tasks/arc104_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    println!("{} {}", (A + B) / 2, (A - B) / 2);
}