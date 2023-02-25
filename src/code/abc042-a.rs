// https://atcoder.jp/contests/abc042/tasks/abc042_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let mut array = [A, B, C];
    array.sort();
    if array[0] == 5 && array[1] == 5 && array[2] == 7 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}