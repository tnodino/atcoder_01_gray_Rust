// https://atcoder.jp/contests/abc027/tasks/abc027_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        l1: usize,
        l2: usize,
        l3: usize,
    }
    let mut line = [l1, l2, l3];
    line.sort();
    if line[0] == line[1] {
        println!("{}", line[2]);
    }
    else {
        println!("{}", line[0]);
    }
}