// https://atcoder.jp/contests/abc067/tasks/abc067_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A % 3 == 0 || B % 3 == 0 || (A + B) % 3 == 0 {
        println!("Possible");
    }
    else {
        println!("Impossible");
    }
}