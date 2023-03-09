// https://atcoder.jp/contests/abc285/tasks/abc285_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = 0;
    for s in S.chars() {
        ans *= 26;
        ans += (s as u8 - 64) as usize;
    }
    println!("{}", ans);
}