// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M1: usize,
        _D1: usize,
        M2: usize,
        _D2: usize,
    }
    if M1 != M2 {
        println!("1");
    }
    else {
        println!("0");
    }
}