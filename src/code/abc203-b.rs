// https://atcoder.jp/contests/abc203/tasks/abc203_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=K {
            ans += i * 100 + j;
        }
    }
    println!("{}", ans);
}