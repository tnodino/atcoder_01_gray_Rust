// https://atcoder.jp/contests/abc104/tasks/abc104_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = "AC";
    let mut cnt = 0;
    for i in 0..S.len() {
        if i == 0 {
            if S[i] != 'A' {
                ans = "WA";
            }
            continue;
        }
        if S[i].is_uppercase() {
            cnt += 1;
            if i == 1 || i == S.len() - 1 || S[i] != 'C' {
                ans = "WA";
            }
        }
    }
    if cnt != 1 {
        ans = "WA";
    }
    println!("{}", ans);
}