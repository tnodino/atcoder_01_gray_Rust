// https://atcoder.jp/contests/abc068/tasks/abc068_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x = 1;
    while x * 2 <= N {
        x *= 2;
    }
    println!("{}", x);
}