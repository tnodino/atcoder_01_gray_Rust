// https://atcoder.jp/contests/abc034/tasks/abc034_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", match n % 2 {
        0 => n - 1,
        1 => n + 1,
        _ => unreachable!()
    });
}