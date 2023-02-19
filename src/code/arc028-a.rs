// https://atcoder.jp/contests/arc028/tasks/arc028_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: isize,
        A: isize,
        B: isize,
    }
    let mut turn = 1;
    while N > 0 {
        turn ^= 1;
        if turn == 0 {
            N -= A;
        }
        else {
            N -= B;
        }
    }
    println!("{}", match turn {
        0 => "Ant",
        1 => "Bug",
        _ => unreachable!()
    });
}