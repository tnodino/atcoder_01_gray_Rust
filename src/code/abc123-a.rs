// https://atcoder.jp/contests/abc123/tasks/abc123_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        k: usize,
    }
    let mut array = [a, b, c, d, e];
    array.sort();
    for i in 0..5 {
        for j in i+1..5 {
            if array[j] - array[i] > k {
                println!(":(");
                return;
            }
        }
    }
    println!("Yay!");
}