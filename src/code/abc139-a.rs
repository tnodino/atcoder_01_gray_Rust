// https://atcoder.jp/contests/abc139/tasks/abc139_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..3 {
        if S[i] == T[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}