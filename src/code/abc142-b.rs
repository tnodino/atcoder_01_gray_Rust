// https://atcoder.jp/contests/abc142/tasks/abc142_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        h: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if h[i] >= K {
            ans += 1;
        }
    }
    println!("{}", ans);
}