// https://atcoder.jp/contests/abc291/tasks/abc291_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..S.len() {
        if S[i].is_uppercase() {
            println!("{}", i + 1);
            return;
        }
    }
}