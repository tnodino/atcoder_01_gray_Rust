// https://atcoder.jp/contests/arc020/tasks/arc020_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    let ans;
    if A.abs() < B.abs() {
        ans = "Ant";
    }
    else if A.abs() > B.abs() {
        ans = "Bug";
    }
    else {
        ans = "Draw";
    }
    println!("{}", ans);
}