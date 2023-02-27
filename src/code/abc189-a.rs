// https://atcoder.jp/contests/abc189/tasks/abc189_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.dedup();
    if S.len() == 1 {
        println!("Won");
    }
    else {
        println!("Lost");
    }
}