// https://atcoder.jp/contests/arc110/tasks/arc110_a

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut lc = 1;
    for i in 2..=N {
        lc = lcm(lc, i);
    }
    println!("{}", lc + 1);
}