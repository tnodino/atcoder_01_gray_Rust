// https://atcoder.jp/contests/abc289/tasks/abc289_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    for v in s.chars() {
        print!("{}", match v {
            '0' => 1,
            _ => 0,
        });
    }
    println!();
}