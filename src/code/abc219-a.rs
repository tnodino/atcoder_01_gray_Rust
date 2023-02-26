// https://atcoder.jp/contests/abc219/tasks/abc219_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    if X < 40 {
        println!("{}", 40 - X);
    }
    else if X < 70 {
        println!("{}", 70 - X);
    }
    else if X < 90 {
        println!("{}", 90 - X);
    }
    else {
        println!("expert");
    }
}