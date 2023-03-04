// https://atcoder.jp/contests/abc138/tasks/abc138_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        s: String,
    }
    if a < 3200 {
        println!("{}", "red");
    }
    else {
        println!("{}", s);
    }
}