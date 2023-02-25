// https://atcoder.jp/contests/abc037/tasks/abc037_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut vec = vec![0; N];
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
            T: usize,
        }
        for i in L-1..R {
            vec[i] = T;
        }
    }
    for i in 0..N {
        println!("{}", vec[i]);
    }
}