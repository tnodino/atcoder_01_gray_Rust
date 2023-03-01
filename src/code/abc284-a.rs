// https://atcoder.jp/contests/abc284/tasks/abc284_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }
    for i in (0..N).rev() {
        println!("{}", S[i]);
    }
}