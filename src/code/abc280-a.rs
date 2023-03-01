// https://atcoder.jp/contests/abc280/tasks/abc280_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        _W: usize,
    }
    let mut ans = 0;
    for _ in 0..H {
        input! {
            S: String,
        }
        ans += S.chars().filter(|&s| s == '#').count();
    }
    println!("{}", ans);
}