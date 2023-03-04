// https://atcoder.jp/contests/abc219/tasks/abc219_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S1: String,
        S2: String,
        S3: String,
        T: String,
    }
    for t in T.chars() {
        print!("{}", match t {
            '1' => S1.clone(),
            '2' => S2.clone(),
            _ => S3.clone(),
        });
    }
    println!();
}