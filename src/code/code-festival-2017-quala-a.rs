// https://atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_quala_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if S.len() >= 4 && &S[0..4] == "YAKI" {
        println!("Yes");
    }
    else {
        println!("No")
    }
}