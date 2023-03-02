// https://atcoder.jp/contests/abc276/tasks/abc276_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in (0..S.len()).rev() {
        if S[i] == 'a' {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}