// https://atcoder.jp/contests/abc171/tasks/abc171_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: String,
    }
    let a = a.chars().next().unwrap();
    if a.is_uppercase() {
        println!("A")
    }
    else {
        println!("a")
    }
}