// https://atcoder.jp/contests/abc048/tasks/abc048_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: [String; 3],
    }
    println!("A{}C", s[1].chars().next().unwrap());
}