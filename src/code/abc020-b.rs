// https://atcoder.jp/contests/abc020/tasks/abc020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        B: String,
    }
    let S = format!("{}{}", A, B);
    let N = S.parse::<usize>().unwrap();
    println!("{}", N * 2);
}