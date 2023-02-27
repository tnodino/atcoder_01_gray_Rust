// https://atcoder.jp/contests/abc240/tasks/abc240_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a % 10 + 1 == b || b % 10 + 1 == a {
        println!("Yes");
    }
    else {
        println!("No");
    }
}