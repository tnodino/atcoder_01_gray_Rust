// https://atcoder.jp/contests/abc073/tasks/abc073_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    if N.contains("9") {
        println!("Yes");
    }
    else {
        println!("No");
    }
}