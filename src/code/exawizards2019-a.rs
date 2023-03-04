// https://atcoder.jp/contests/exawizards2019/tasks/exawizards2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    if A == B && A == C {
        println!("Yes");
    }
    else {
        println!("No");
    }
}