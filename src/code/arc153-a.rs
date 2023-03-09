// https://atcoder.jp/contests/arc153/tasks/arc153_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    N -= 1;
    let mut array = [0; 6];
    for i in (0..6).rev() {
        array[i] = N % 10;
        N /= 10;
    }
    array[0] += 1;
    for i in 0..9 {
        print!("{}", match i {
            0 => array[0],
            1 => array[0],
            2 => array[1],
            3 => array[2],
            4 => array[3],
            5 => array[3],
            6 => array[4],
            7 => array[5],
            _ => array[4],
        });
    }
    println!();
}