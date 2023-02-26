// https://atcoder.jp/contests/abc056/tasks/abc056_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: String,
        b: String,
    }
    if a == b {
        println!("H");
    }
    else {
        println!("D");
    }
}