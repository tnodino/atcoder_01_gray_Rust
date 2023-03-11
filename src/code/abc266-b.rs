// https://atcoder.jp/contests/abc266/tasks/abc266_b

use proconio::input;
use proconio::fastout;

const MOD: isize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
    }
    println!("{}", (N % MOD + MOD) % MOD);
}