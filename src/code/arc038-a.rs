// https://atcoder.jp/contests/arc038/tasks/arc038_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    for i in (0..N).step_by(2) {
        ans += A[i];
    }
    println!("{}", ans);
}