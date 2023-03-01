// https://atcoder.jp/contests/abc285/tasks/abc285_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if b / 2 == a {
        println!("Yes");
    }
    else {
        println!("No");
    }
}