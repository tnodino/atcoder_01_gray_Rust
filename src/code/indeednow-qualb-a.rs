// https://atcoder.jp/contests/indeednow-qualb/tasks/indeednow_2015_qualb_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }
    println!("{}", (x1 - x2).abs() + (y1 - y2).abs() + 1);
}