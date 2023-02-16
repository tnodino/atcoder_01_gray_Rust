// https://atcoder.jp/contests/arc010/tasks/arc010_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: isize,
        M: usize,
        A: isize,
        B: isize,
    }
    for i in 1..=M {
        if N <= A {
            N += B;
        }
        input! {
            c: isize,
        }
        N -= c;
        if N < 0 {
            println!("{}", i);
            return;
        }
    }
    println!("complete");
}