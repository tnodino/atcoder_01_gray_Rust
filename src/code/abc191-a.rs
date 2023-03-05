// https://atcoder.jp/contests/abc191/tasks/abc191_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        V: usize,
        T: usize,
        S: usize,
        D: usize,
    }
    if V * T <= D && D <= V * S {
        println!("No");
    }
    else {
        println!("Yes");
    }
}