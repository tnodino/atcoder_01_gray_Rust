// https://atcoder.jp/contests/code-festival-2014-quala/tasks/code_festival_qualA_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        mut B: usize,
    }
    let len = A.len();
    B -= 1;
    B %= len;
    println!("{}", A.chars().nth(B).unwrap());
}