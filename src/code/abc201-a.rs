// https://atcoder.jp/contests/abc201/tasks/abc201_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        A3: usize,
    }
    let mut array = [A1, A2, A3];
    array.sort();
    if array[1] - array[0] == array[2] - array[1] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}