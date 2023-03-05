// https://atcoder.jp/contests/abc129/tasks/abc129_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: [isize; N],
    }
    let mut sum = W.iter().sum::<isize>();
    let mut cnt = 0;
    let mut ans = sum;
    for i in 0..N {
        sum -= W[i];
        cnt += W[i];
        ans = min(ans, (sum - cnt).abs());
    }
    println!("{}", ans);
}