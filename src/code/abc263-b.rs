// https://atcoder.jp/contests/abc263/tasks/abc263_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N-1],
    }
    let mut idx = N - 1;
    let mut ans = 0;
    while idx > 0 {
        idx = P[idx-1] - 1;
        ans += 1;
    }
    println!("{}", ans);
}