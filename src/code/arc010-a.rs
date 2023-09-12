// https://atcoder.jp/contests/arc010/tasks/arc010_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (mut N, M, A, B): (usize, usize, usize, usize),
    }
    for i in 1..=M {
        if N <= A {
            N += B;
        }
        input! {
            c: usize,
        }
        if N < c {
            println!("{}", i);
            return;
        }
        N -= c;
    }
    println!("complete");
}