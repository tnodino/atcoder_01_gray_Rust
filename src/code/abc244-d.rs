// https://atcoder.jp/contests/abc244/tasks/abc244_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S1: char,
        S2: char,
        S3: char,
        T1: char,
        T2: char,
        T3: char,
    }
    if (S1 == T1 && S2 == T2 && S3 == T3) || (S1 != T1 && S2 != T2 && S3 != T3) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}