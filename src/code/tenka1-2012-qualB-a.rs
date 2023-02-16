// https://atcoder.jp/contests/tenka1-2012-qualB/tasks/tenka1_2012_5

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    for i in 1..=127 {
        if i % 3 == a && i % 5 == b && i % 7 == c {
            println!("{}", i);
        }
    }
}