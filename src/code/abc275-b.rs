// https://atcoder.jp/contests/abc275/tasks/abc275_b

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
        D: usize,
        E: usize,
        F: usize,
    }
    let X = (A % MOD) * (B % MOD) % MOD * (C % MOD) % MOD;
    let Y = (D % MOD) * (E % MOD) % MOD * (F % MOD) % MOD;
    println!("{}", (X + MOD - Y) % MOD);
}