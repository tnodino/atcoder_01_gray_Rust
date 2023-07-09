// https://atcoder.jp/contests/abc309/tasks/abc309_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
    }
    A -= 1;
    B -= 1;
    if A / 3 == B / 3 && (A % 3) + 1 == B % 3 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}