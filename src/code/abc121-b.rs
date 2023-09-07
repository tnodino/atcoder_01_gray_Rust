// https://atcoder.jp/contests/abc121/tasks/abc121_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, C): (usize, usize, isize),
        B: [isize; M],
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: [isize; M],
        }
        let mut res = C;
        for i in 0..M {
            res += A[i] * B[i];
        }
        if res > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}