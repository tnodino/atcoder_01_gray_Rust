// https://atcoder.jp/contests/abc318/tasks/abc318_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut M : usize,
        P: usize,
    }
    let mut ans = 0;
    while M <= N {
        ans += 1;
        M += P;
    }
    println!("{}", ans);
}