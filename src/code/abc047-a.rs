// https://atcoder.jp/contests/abc047/tasks/abc047_a

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
    if array[0] + array[1] == array[2] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}