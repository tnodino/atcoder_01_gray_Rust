// https://atcoder.jp/contests/abc318/tasks/abc318_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 100;
    let mut vec = vec![vec![false; M]; M];
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
            C: usize,
            D: usize,
        }
        for i in A..B {
            for j in C..D {
                vec[i][j] |= true;
            }
        }
    }
    let mut ans = 0;
    for i in 0..M {
        for j in 0..M {
            ans += vec[i][j] as usize;
        }
    }
    println!("{}", ans);
}