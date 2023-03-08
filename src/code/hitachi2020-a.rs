// https://atcoder.jp/contests/hitachi2020/tasks/hitachi2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    S = S.replace("hi", "-");
    for s in S.chars() {
        if s != '-' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}