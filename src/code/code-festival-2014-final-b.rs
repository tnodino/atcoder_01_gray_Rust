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
    let mut flg = 0;
    for s in S.chars() {
        if flg == 0 {
            ans += s as isize - 48;
        }
        else {
            ans -= s as isize - 48;
        }
        flg ^= 1;
    }
    println!("{}", ans);
}