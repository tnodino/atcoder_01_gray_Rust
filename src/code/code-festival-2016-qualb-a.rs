// https://atcoder.jp/contests/code-festival-2016-qualb/tasks/codefestival_2016_qualB_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = "CODEFESTIVAL2016".chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..S.len() {
        if S[i] != T[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}