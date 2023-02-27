// https://atcoder.jp/contests/tenka1-2012-qualA/tasks/tenka1_2012_qualA_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut DP = vec![1, 1];
    for i in 2..=n {
        DP.push(DP[i-1] + DP[i-2]);
    }
    println!("{}", DP[n]);
}