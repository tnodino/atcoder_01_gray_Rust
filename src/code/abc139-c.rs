// https://atcoder.jp/contests/abc139/tasks/abc139_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }
    let mut ans = 0;
    let mut cnt = 0;
    for i in 1..N {
        if H[i-1] >= H[i] {
            cnt += 1;
        }
        else {
            ans = max(ans, cnt);
            cnt = 0;
        }
    }
    println!("{}", max(ans, cnt));
}