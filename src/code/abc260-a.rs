// https://atcoder.jp/contests/abc260/tasks/abc260_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    for s in S.chars() {
        if S.chars().filter(|&x| x == s).count() == 1 {
            println!("{}", s);
            return;
        }
    }
    println!("-1");
}