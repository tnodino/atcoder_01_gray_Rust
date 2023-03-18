// https://atcoder.jp/contests/abc200/tasks/abc200_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut cnt = [0; 200];
    for i in 0..N {
        cnt[A[i] % 200] += 1;
    }
    let mut ans: isize = 0;
    for i in 0..200 {
        ans += (cnt[i] - 1) * cnt[i] / 2;
    }
    println!("{}", ans);
}