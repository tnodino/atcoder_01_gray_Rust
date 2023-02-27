// https://atcoder.jp/contests/abc084/tasks/abc084_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        _B: usize,
        S: String,
    }
    if S.chars().enumerate().all(|(i, s)| (i != A && s.is_ascii_digit()) || (i == A && s == '-')) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}