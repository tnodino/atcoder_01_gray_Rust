// https://atcoder.jp/contests/abc251/tasks/abc251_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = S.clone();
    while ans.len() < 6 {
        ans = format!("{}{}", ans, S);
    }
    println!("{}", ans);
}