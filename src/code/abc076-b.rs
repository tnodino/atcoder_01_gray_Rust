// https://atcoder.jp/contests/abc076/tasks/abc076_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut ans = 1;
    for _ in 0..N {
        if ans * 2 < ans + K {
            ans *= 2;
        }
        else {
            ans += K;
        }
    }
    println!("{}", ans);
}