// https://atcoder.jp/contests/arc107/tasks/arc107_a

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let a = (A + 1) * A / 2 % MOD;
    let b = (B + 1) * B / 2 % MOD;
    let c = (C + 1) * C / 2 % MOD;
    println!("{}", a * b % MOD * c % MOD);
}