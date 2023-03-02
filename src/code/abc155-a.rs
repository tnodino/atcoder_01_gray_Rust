// https://atcoder.jp/contests/abc155/tasks/abc155_a

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
    if array[0] == array[1] && array[0] != array[2] {
        println!("Yes");
    }
    else if array[1] == array[2] && array[0] != array[2] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}