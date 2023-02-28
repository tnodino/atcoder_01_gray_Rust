// https://atcoder.jp/contests/abc088/tasks/abc088_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut alice = 0;
    let mut bob = 0;
    for (i, v) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += v;
        }
        else {
            bob += v;
        }
    }
    println!("{}", alice - bob);
}