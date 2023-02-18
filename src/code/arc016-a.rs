// https://atcoder.jp/contests/arc016/tasks/arc016_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut ans = 1;
    if M == 1 {
        ans = N;
    }
    println!("{}", ans);
}