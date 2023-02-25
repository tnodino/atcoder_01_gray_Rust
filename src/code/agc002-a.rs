// https://atcoder.jp/contests/agc002/tasks/agc002_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
    }
    if a <= 0 && 0 <= b {
        println!("Zero");
    }
    else if 1 <= a {
        println!("Positive");
    }
    else {
        if (b - a + 1) % 2 == 0 {
            println!("Positive");
        }
        else {
            println!("Negative");
        }
    }
}