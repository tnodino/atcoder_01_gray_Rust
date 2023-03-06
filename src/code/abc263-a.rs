// https://atcoder.jp/contests/abc263/tasks/abc263_a

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
        E: usize,
    }
    let mut array = [A, B, C, D, E];
    array.sort();
    if array[0] == array[1] && array[1] == array[2] && array[3] == array[4] {
        println!("Yes");
    }
    else if array[0] == array[1] && array[2] == array[3] && array[3] == array[4] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}