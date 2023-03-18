// https://atcoder.jp/contests/keyence2021/tasks/keyence2021_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
        b: [usize; N],
    }
    let mut an = 0;
    let mut ans = 0;
    for i in 0..N {
        an = max(an, a[i]);
        ans = max(ans, an * b[i]);
        println!("{}", ans);
    }
}