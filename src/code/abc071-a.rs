// https://atcoder.jp/contests/abc071/tasks/abc071_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize,
    }
    if (x - a).abs() < (x - b).abs() {
        println!("A");
    }
    else {
        println!("B");
    }
}