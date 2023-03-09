// https://atcoder.jp/contests/abc228/tasks/abc228_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
        T: usize,
        X: usize,
    }
    if S < T {
        if S <= X && X < T {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
    else {
        if T <= X && X < S {
            println!("No");
        }
        else {
            println!("Yes");
        }
    }
}