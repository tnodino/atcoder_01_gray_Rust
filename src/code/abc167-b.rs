// https://atcoder.jp/contests/abc167/tasks/abc167_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        _C: isize,
        mut K: isize,
    }
    let mut ans = 0;
    if K <= A {
        ans += K;
        K = 0;
    }
    else {
        ans += A;
        K -= A;
    }
    if K <= B {
        K = 0;
    }
    else {
        K -= B;
    }
    ans -= K;
    println!("{}", ans);
}