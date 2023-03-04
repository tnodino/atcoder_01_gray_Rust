// https://atcoder.jp/contests/abc109/tasks/abc109_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A % 2 == 0 || B % 2 == 0 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}