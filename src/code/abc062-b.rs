// https://atcoder.jp/contests/abc062/tasks/abc062_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    println!("{}", "#".repeat(W+2));
    for _ in 0..H {
        print!("#");
        input! {
            a: String,
        }
        print!("{}", a);
        println!("#");
    }
    println!("{}", "#".repeat(W+2));
}