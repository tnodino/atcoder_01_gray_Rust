// https://atcoder.jp/contests/arc154/tasks/arc154_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: String,
        B: String,
    }
    let A = A.chars().collect::<Vec<char>>();
    let B = B.chars().collect::<Vec<char>>();
    let mut a = 0;
    let mut b = 0;
    for i in 0..N {
        let x = A[i] as usize - 48;
        let y = B[i] as usize - 48;
        a *= 10;
        b *= 10;
        a += max(x, y);
        b += min(x, y);
        a %= MOD;
        b %= MOD;
    }
    println!("{}", a * b % MOD);
}