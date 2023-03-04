// https://atcoder.jp/contests/abc121/tasks/abc121_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        C: isize,
        B: [isize; M],
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: [isize; M],
        }
        let mut cnt = C;
        for i in 0..M {
            cnt += A[i] * B[i];
        }
        if cnt > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}