// https://atcoder.jp/contests/abc248/tasks/abc248_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = "0123456789".chars().collect::<Vec<char>>();
    for i in 0..10 {
        if !S.contains(&T[i]) {
            println!("{}", T[i]);
            return;
        }
    }
}