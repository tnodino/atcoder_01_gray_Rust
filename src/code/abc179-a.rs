// https://atcoder.jp/contests/abc179/tasks/abc179_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if S.chars().last().unwrap() == 's' {
        println!("{}es", S);
    }
    else {
        println!("{}s", S);
    }
}