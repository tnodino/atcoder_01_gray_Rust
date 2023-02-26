// https://atcoder.jp/contests/abc197/tasks/abc197_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}{}", &S[1..3], &S[0..1]);
}