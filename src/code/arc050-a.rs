// https://atcoder.jp/contests/arc050/tasks/arc050_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        C: String,
        c: String,
    }
    if C.to_lowercase() == c {
        println!("Yes");
    }
    else {
        println!("No");
    }
}