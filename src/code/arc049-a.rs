// https://atcoder.jp/contests/arc049/tasks/arc049_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    let array = [A, B, C, D];
    let mut idx = 0;
    let mut i = 0;
    for s in S.chars() {
        if idx < 4 && array[idx] == i {
            print!("\"");
            idx += 1;
        }
        print!("{}", s);
        i += 1;
    }
    if idx == 3 {
        print!("\"");
    }
    println!("");
}