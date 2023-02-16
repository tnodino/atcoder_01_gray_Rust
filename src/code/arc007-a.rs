// https://atcoder.jp/contests/arc007/tasks/arc007_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
        s: String,
    }
    println!("{}", s.replace(&X, ""));
}