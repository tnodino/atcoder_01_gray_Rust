// https://atcoder.jp/contests/abc166/tasks/abc166_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        H: [usize; N],
    }
    let mut flg = vec![true; N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        if H[A-1] <= H[B-1] {
            flg[A-1] = false;
        }
        if H[A-1] >= H[B-1] {
            flg[B-1] = false;
        }
    }
    println!("{}", flg.iter().filter(|&x| *x).count());
}