// https://atcoder.jp/contests/abc166/tasks/abc166_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut flg = vec![true; N];
    for _ in 0..K {
        input! {
            d: usize,
            A: [usize; d],
        }
        for a in A {
            flg[a-1] = false;
        }
    }
    println!("{}", flg.iter().filter(|&x| *x).count());
}