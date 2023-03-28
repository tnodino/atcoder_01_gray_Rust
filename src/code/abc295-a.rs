// https://atcoder.jp/contests/abc295/tasks/abc295_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let arr = ["and".to_string(), "not".to_string(), "that".to_string(), "the".to_string(), "you".to_string()];
    for _ in 0..N {
        input! {
            W: String,
        }
        if arr.contains(&W) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}