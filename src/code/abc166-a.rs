// https://atcoder.jp/contests/abc166/tasks/abc166_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", match S.as_str() {
        "ABC" => "ARC",
        _ => "ABC",
    });
}