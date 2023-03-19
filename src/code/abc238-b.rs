// https://atcoder.jp/contests/abc238/tasks/abc238_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut set = vec![0, 360];
    let mut now = 0;
    for a in A {
        now += a;
        now %= 360;
        set.push(now);
    }
    set.sort();
    let mut ans = 0;
    for i in 0..N+1 {
        ans = max(ans, set[i+1] - set[i]);
    }
    println!("{}", ans);
}