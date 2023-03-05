// https://atcoder.jp/contests/abc290/tasks/abc290_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        K: usize,
        S: String,
    }
    let mut cnt = 0;
    for s in S.chars() {
        if s == 'o' && cnt < K {
            print!("o");
            cnt += 1;
        }
        else {
            print!("x");
        }
    }
    println!();
}