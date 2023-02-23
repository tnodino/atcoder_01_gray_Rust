// https://atcoder.jp/contests/abc007/tasks/abc007_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
    }
    println!("{}", match A.as_str() {
        "a" => "-1",
        _ => "a",
    });
}