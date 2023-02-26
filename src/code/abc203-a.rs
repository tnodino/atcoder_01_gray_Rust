// https://atcoder.jp/contests/abc203/tasks/abc203_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut array = [a, b, c];
    array.sort();
    if array[0] == array[1] {
        println!("{}", array[2]);
    }
    else if array[1] == array[2] {
        println!("{}", array[0]);
    }
    else {
        println!("0")
    }
}