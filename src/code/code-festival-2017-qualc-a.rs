// https://atcoder.jp/contests/code-festival-2017-qualc/tasks/code_festival_2017_qualc_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    for i in 0..S.len()-1 {
        if &S[i..i+2] == "AC" {
            println!("Yes");
            return;
        }
    }
    println!("No");
}