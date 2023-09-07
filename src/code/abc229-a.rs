// https://atcoder.jp/contests/abc229/tasks/abc229_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S1: String,
        S2: String,
    }
    if S1 == ".#" && S2 == "#." {
        println!("No");
    }
    else if S1 == "#." && S2 == ".#" {
        println!("No");
    }
    else {
        println!("Yes");
    }
}