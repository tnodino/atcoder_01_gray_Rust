// https://atcoder.jp/contests/abc185/tasks/abc185_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        T: usize,
    }
    let mut mah = N;
    let mut time = 0;
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        if A - time >= mah {
            println!("No");
            return;
        }
        mah -= A - time;
        mah += B - A;
        mah = min(mah, N);
        time = B;
    }
    if T - time >= mah {
        println!("No");
    }
    else {
        println!("Yes");
    }
}