// https://atcoder.jp/contests/arc105/tasks/arc105_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    let mut array = [A, B, C, D];
    array.sort();
    if array[0] + array[3] == array[1] + array[2] || array[0] + array[1] + array[2] == array[3] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}