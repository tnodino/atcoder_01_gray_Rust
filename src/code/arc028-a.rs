// https://atcoder.jp/contests/arc028/tasks/arc028_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (mut N, A, B): (usize, usize, usize),
    }
    loop {
        if N <= A {
            println!("Ant");
            return;
        }
        N -= A;
        if N <= B {
            println!("Bug");
            return;
        }
        N -= B;
    }
}