// https://atcoder.jp/contests/abc061/tasks/abc061_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
    }
    if A <= C && C <= B {
        println!("Yes");
    }
    else {
        println!("No");
    }
}