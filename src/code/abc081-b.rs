// https://atcoder.jp/contests/abc081/tasks/abc081_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans: usize = !0;
    for i in 0..N {
        let mut x = A[i];
        let mut cnt = 0;
        while x % 2 == 0 {
            x /= 2;
            cnt += 1;
        }
        ans = min(ans, cnt);
    }
    println!("{}", ans)

}