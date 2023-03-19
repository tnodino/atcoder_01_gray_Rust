// https://atcoder.jp/contests/arc132/tasks/arc132_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        R: [usize; n],
        C: [usize; n],
        q: usize,
    }
    for _ in 0..q {
        input! {
            r: usize,
            c: usize,
        }
        if R[r-1] + C[c-1] >= n + 1 {
            print!("#");
        }
        else {
            print!(".");
        }
    }
    println!();
}