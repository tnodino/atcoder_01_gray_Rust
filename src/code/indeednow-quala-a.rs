// https://atcoder.jp/contests/indeednow-quala/tasks/indeednow_2015_quala_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        B: String,
    }
    println!("{}", A.len() * B.len());
}