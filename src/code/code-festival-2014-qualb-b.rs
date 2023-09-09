// https://atcoder.jp/contests/code-festival-2014-qualb/tasks/code_festival_qualB_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
    }
    let mut sum = 0;
    for i in 1..=N {
        input! {
            a: usize,
        }
        sum += a;
        if sum >= K {
            println!("{}", i);
            return;
        }
    }
}