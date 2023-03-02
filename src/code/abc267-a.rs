// https://atcoder.jp/contests/abc267/tasks/abc267_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", match S.as_str() {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        _ => 1,
    });
}