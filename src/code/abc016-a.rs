// https://atcoder.jp/contests/abc016/tasks/abc016_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        D: usize,
    }
    let ans;
    if M % D == 0 {
        ans = "YES";
    }
    else {
        ans = "NO";
    }
    println!("{}", ans);
}