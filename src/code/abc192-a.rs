// https://atcoder.jp/contests/abc192/tasks/abc192_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
    }
    println!("{}", match X % 100 {
        0 => 100,
        _ => 100 - X % 100,
    });
}