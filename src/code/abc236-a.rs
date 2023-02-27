// https://atcoder.jp/contests/abc236/tasks/abc236_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        a: usize,
        b: usize,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.swap(a-1, b-1);
    println!("{}", S.iter().collect::<String>());
}