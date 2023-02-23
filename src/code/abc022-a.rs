// https://atcoder.jp/contests/abc022/tasks/abc022_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: isize,
        T: isize,
    }
    let mut W = 0;
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: isize,
        }
        W += A;
        if S <= W && W <= T {
            ans += 1;
        }
    }
    println!("{}", ans);
}