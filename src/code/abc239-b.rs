// https://atcoder.jp/contests/abc239/tasks/abc239_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
    }
    if 0 <= X {
        println!("{}", X / 10);
    }
    else {
        println!("{}", (X - 9) / 10);
    }
}