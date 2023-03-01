// https://atcoder.jp/contests/abc091/tasks/abc091_a

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
    if A + B >= C {
        println!("Yes");
    }
    else {
        println!("No");
    }
}