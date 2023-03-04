// https://atcoder.jp/contests/abc247/tasks/abc247_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    print!("0");
    for i in 0..3 {
        print!("{}", S[i]);
    }
    println!();
}