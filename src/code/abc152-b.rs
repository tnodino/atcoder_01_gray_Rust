// https://atcoder.jp/contests/abc152/tasks/abc152_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a < b {
        println!("{}", a.to_string().repeat(b));
    }
    else {
        println!("{}", b.to_string().repeat(a));
    }
}