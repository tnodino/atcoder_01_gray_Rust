// https://atcoder.jp/contests/abc013/tasks/abc013_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
    }
    let mut now = a as isize;
    let mut Red = 0;
    while now != b {
        now += 1;
        now %= 10;
        Red += 1;
    }
    let mut now = a as isize;
    let mut Blue = 0;
    while now != b {
        now -= 1;
        now += 10;
        now %= 10;
        Blue += 1;
    }
    println!("{}", Red.min(Blue));
}