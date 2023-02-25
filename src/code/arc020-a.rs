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
    if A.abs() < B.abs() {
        println!("Ant");
    }
    else if A.abs() > B.abs() {
        println!("Bug");
    }
    else {
        println!("Draw");
    }
}