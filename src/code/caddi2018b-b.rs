// https://atcoder.jp/contests/caddi2018b/tasks/caddi2018b_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: usize,
        W: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        if A >= H && B >= W {
            ans += 1;
        }
    }
    println!("{}", ans);
}