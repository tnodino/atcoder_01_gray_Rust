// https://atcoder.jp/contests/abc029/tasks/abc029_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut ans = 0;
    for _ in 0..12 {
        input! {
            S: String,
        }
        if S.contains("r") {
            ans += 1;
        }
    }
    println!("{}", ans);
}