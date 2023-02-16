// https://atcoder.jp/contests/arc002/tasks/arc002_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Y: usize,
    }
    let mut ans = "NO";
    if Y % 4 == 0 {
        ans = "YES";
    }
    if Y % 100 == 0 {
        ans = "NO";
    }
    if Y % 400 == 0 {
        ans = "YES";
    }
    println!("{}", ans);
}