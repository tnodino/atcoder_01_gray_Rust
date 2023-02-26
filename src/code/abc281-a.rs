// https://atcoder.jp/contests/abc281/tasks/abc281_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for i in (0..=N).rev() {
        println!("{}", i);
    }
}