// https://atcoder.jp/contests/abc078/tasks/abc078_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
        Y: String,
    }
    if X < Y {
        println!("<");
    }
    else if X > Y {
        println!(">");
    }
    else {
        println!("=");
    }
}