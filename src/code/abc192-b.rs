// https://atcoder.jp/contests/abc192/tasks/abc192_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    for (i, s) in S.chars().enumerate() {
        if i % 2 == 0 {
            if !s.is_lowercase() {
                println!("No");
                return;
            }
        }
        else {
            if !s.is_uppercase() {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}