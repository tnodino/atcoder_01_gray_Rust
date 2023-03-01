// https://atcoder.jp/contests/zone2021/tasks/zone2021_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = 0;
    for i in 0..S.len()-3 {
        if &S[i..i+4] == "ZONe" {
            ans += 1;
        }
    }
    println!("{}", ans);
}