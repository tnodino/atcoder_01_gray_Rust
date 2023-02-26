// https://atcoder.jp/contests/abc069/tasks/abc069_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    println!("{}", (n - 1) * (m - 1));
}