// https://atcoder.jp/contests/tenka1-2018-beginner/tasks/tenka1_2018_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
        K: usize,
    }
    for i in 0..K {
        if i % 2 == 0 {
            if A % 2 == 1 {
                A -= 1;
            }
            B += A / 2;
            A /= 2;
        }
        else {
            if B % 2 == 1 {
                B -= 1;
            }
            A += B / 2;
            B /= 2;
        }
    }
    println!("{} {}", A, B);
}