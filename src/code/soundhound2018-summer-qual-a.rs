// https://atcoder.jp/contests/soundhound2018-summer-qual/tasks/soundhound2018_summer_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a + b == 15 {
        println!("+");
    }
    else if a * b == 15 {
        println!("*");
    }
    else {
        println!("x");
    }
}