// https://atcoder.jp/contests/arc037/tasks/arc037_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        m: [isize; N],
    }
    let mut ans = 0;
    for x in m {
        ans += max(80-x, 0);
    }
    println!("{}", ans);
}