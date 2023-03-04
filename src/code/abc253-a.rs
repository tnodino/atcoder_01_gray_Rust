// https://atcoder.jp/contests/abc253/tasks/abc253_a

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
    if b == array[1] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}