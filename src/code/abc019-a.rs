// https://atcoder.jp/contests/abc019/tasks/abc019_1

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
    println!("{}", array[1]);
}