// https://atcoder.jp/contests/abc281/tasks/abc281_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if S.len() != 8 {
        println!("No");
        return;
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = "Yes";
    for i in 0..8 {
        if i == 0 || i == 7 {
            if !S[i].is_uppercase() {
                ans = "No";
            }
        }
        else if i == 1 {
            if S[i] == '0' {
                ans = "No";
            }
        }
        else {
            if !S[i].is_numeric() {
                ans = "No";
            }
        }
    }
    println!("{}", ans);
}