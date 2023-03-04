// https://atcoder.jp/contests/caddi2018b/tasks/caddi2018b_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    println!("{}", N.chars().filter(|&x| x == '2').count());
}