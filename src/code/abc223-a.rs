// https://atcoder.jp/contests/abc223/tasks/abc223_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    if X % 100 == 0 && X > 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}