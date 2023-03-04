// https://atcoder.jp/contests/abc211/tasks/abc211_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S1: String,
        S2: String,
        S3: String,
        S4: String,
    }
    let mut vec = vec![S1, S2, S3, S4];
    vec.sort();
    vec.dedup();
    if vec.len() == 4 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}