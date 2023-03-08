// https://atcoder.jp/contests/abc279/tasks/abc279_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    if S.contains(&T) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}