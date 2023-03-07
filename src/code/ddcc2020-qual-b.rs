// https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut l = 0;
    let mut r = A.iter().sum::<isize>();
    let mut ans = r;
    for i in 0..N-1 {
        l += A[i];
        r -= A[i];
        ans = min(ans, (r - l).abs());
    }
    println!("{}", ans);
}