// https://atcoder.jp/contests/abc217/tasks/abc217_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    if S < T {
        println!("Yes");
    }
    else {
        println!("No");
    }
}