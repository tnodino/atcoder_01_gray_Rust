// https://atcoder.jp/contests/abc001/tasks/abc001_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H1: isize,
        H2: isize,
    }
    println!("{}", H1 - H2);
}