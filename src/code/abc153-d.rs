// https://atcoder.jp/contests/abc153/tasks/abc153_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut H: usize,
    }
    let mut x: usize = 1;
    let mut ans: usize = 0;
    while H > 0 {
        ans += x;
        H /= 2;
        x *= 2;
    }
    println!("{}", ans);
}