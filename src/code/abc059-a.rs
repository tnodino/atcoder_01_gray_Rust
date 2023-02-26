// https://atcoder.jp/contests/abc059/tasks/abc059_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }
    let s = [s1, s2, s3];
    for i in 0..3 {
        print!("{}", s[i].chars().next().unwrap().to_uppercase());
    }
    println!();
}