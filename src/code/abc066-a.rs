// https://atcoder.jp/contests/abc066/tasks/abc066_a

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
    println!("{}", array[0] + array[1]);
}