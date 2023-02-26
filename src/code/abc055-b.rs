// https://atcoder.jp/contests/abc055/tasks/abc055_b

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {

    input! {
        N: usize,
    }
    let mut ans = 1;
    for i in 1..=N {
        ans *= i;
        ans %= MOD;
    }
    println!("{}", ans);
}