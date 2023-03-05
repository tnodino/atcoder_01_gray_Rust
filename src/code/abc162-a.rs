// https://atcoder.jp/contests/abc162/tasks/abc162_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    if N.contains("7") {
        println!("Yes");
    }
    else {
        println!("No");
    }
}