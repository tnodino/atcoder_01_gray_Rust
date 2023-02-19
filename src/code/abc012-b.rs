// https://atcoder.jp/contests/abc012/tasks/abc012_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let h = N / 3600;
    let m = N / 60 - h * 60;
    let s = N % 60;
    println!("{:02}:{:02}:{:02}", h, m, s);
}