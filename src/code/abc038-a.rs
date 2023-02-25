// https://atcoder.jp/contests/abc038/tasks/abc038_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", match S.chars().last().unwrap() {
        'T' => "YES",
        _ => "NO",
    });
}