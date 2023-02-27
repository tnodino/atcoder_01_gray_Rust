// https://atcoder.jp/contests/abc075/tasks/abc075_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
    }
    let mut array = [A, B, C];
    array.sort();
    if array[0] == array[1] {
        println!("{}", array[2]);
    }
    else {
        println!("{}", array[0]);
    }
}