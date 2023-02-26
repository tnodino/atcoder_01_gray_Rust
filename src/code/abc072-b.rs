// https://atcoder.jp/contests/abc072/tasks/abc072_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    println!("{}", s.chars().step_by(2).collect::<String>());
}