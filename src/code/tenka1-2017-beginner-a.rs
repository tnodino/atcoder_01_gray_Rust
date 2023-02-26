// https://atcoder.jp/contests/tenka1-2017-beginner/tasks/tenka1_2017_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", S.chars().filter(|&c| c == '1').count());
}