// https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_a

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
    if (A < C && C < B) || (B < C && C < A) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}