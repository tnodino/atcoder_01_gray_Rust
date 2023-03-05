// https://atcoder.jp/contests/abc238/tasks/abc238_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    if 2 <= n && n <= 4 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}