// https://atcoder.jp/contests/abc237/tasks/abc237_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
    }
    if -(1 << 31) <= N && N < 1 << 31 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}