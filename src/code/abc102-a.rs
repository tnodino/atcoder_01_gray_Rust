// https://atcoder.jp/contests/abc102/tasks/abc102_a

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", lcm(2, N));
}