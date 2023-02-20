// https://atcoder.jp/contests/abc015/tasks/abc015_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        B: String,
    }
    let ans;
    if A.len() > B.len() {
        ans = A;
    }
    else {
        ans = B;
    }
    println!("{}", ans);
}