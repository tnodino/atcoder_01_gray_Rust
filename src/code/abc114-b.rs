// https://atcoder.jp/contests/abc114/tasks/abc114_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = 999;
    for i in 0..S.len()-2 {
        let X = &S[i..i+3].parse::<isize>().unwrap();
        ans = min(ans, (753 - X).abs());
    }
    println!("{}", ans);
}