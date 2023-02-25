// https://atcoder.jp/contests/abc028/tasks/abc028_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N < 60 {
        println!("Bad");
    }
    else if N < 90 {
        println!("Good");
    }
    else if N < 100 {
        println!("Great");
    }
    else {
        println!("Perfect");
    }
}