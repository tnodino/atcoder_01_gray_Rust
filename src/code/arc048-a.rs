// https://atcoder.jp/contests/arc048/tasks/arc048_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    let mut flg = 0;
    if A < 0 && 0 < B {
        flg = -1;
    }
    println!("{}", B - A + flg);
}