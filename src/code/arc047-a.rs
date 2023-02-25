// https://atcoder.jp/contests/arc047/tasks/arc047_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        L: usize,
        S: String,
    }
    let mut cnt = 1;
    let mut ans = 0;
    for s in S.chars() {
        if s == '+' {
            cnt += 1;
        }
        else {
            cnt -= 1;
        }
        if cnt > L {
            cnt = 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}