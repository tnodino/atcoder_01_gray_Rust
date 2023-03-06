// https://atcoder.jp/contests/abc208/tasks/abc208_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A <= B && B <= A * 6 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}