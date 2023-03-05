// https://atcoder.jp/contests/abc217/tasks/abc217_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S1: String,
        S2: String,
        S3: String,
    }
    let S = [S1, S2, S3];
    let T = ["ABC".to_string(), "ARC".to_string(), "AGC".to_string(), "AHC".to_string()];
    for i in 0..4 {
        if !S.contains(&T[i]) {
            println!("{}", T[i]);
            return;
        }
    }
}