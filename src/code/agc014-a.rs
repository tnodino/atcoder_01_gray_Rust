// https://atcoder.jp/contests/agc014/tasks/agc014_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
        mut C: usize,
    }
    if A == B && B == C && A % 2 == 0 {
        println!("-1");
    }
    else {
        let mut ans = 0;
        while A % 2 == 0 && B % 2 == 0 && C % 2 == 0 {
            ans += 1;
            let x = B / 2 + C / 2;
            let y = A / 2 + C / 2;
            let z = A / 2 + B / 2;
            A = x;
            B = y;
            C = z;
        }
        println!("{}", ans);
    }
}