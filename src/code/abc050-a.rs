// https://atcoder.jp/contests/abc050/tasks/abc050_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        op: String,
        B: isize,
    }
    println!("{}", match op.as_str() {
        "+" => A + B,
        "-" => A - B,
        _ => unreachable!()
    });
}