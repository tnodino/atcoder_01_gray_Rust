// https://atcoder.jp/contests/arc041/tasks/arc041_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
        y: usize,
        k: usize,
    }
    if k <= y {
        println!("{}", x + k);
    }
    else {
        println!("{}", x - (k - y) + y);
    }
}