// https://atcoder.jp/contests/abc111/tasks/abc111_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N + 110) / 111 * 111);
}