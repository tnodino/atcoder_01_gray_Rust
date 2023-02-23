// https://atcoder.jp/contests/abc021/tasks/abc021_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N);
    for _ in 0..N {
        println!("1");
    }
}