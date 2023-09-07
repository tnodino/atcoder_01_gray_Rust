// https://atcoder.jp/contests/code-festival-2014-final/tasks/code_festival_final_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = 0;
    for (i, s) in S.chars().enumerate() {
        let x = (s as isize) - ('0' as isize);
        ans += match i % 2 == 0 {
            true => x,
            false => -x,
        }
    }
    println!("{}", ans);
}