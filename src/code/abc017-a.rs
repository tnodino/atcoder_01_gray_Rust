// https://atcoder.jp/contests/abc017/tasks/abc017_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut ans = 0;
    for _ in 0..3 {
        input! {
            s: usize,
            e: usize,
        }
        ans += s * e / 10;
    }
    println!("{}", ans);
}