// https://atcoder.jp/contests/abc271/tasks/abc271_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            L: usize,
            a: [usize; L],
        }
        A.push(a);
    }
    for _ in 0..Q {
        input! {
            s: usize,
            t: usize,
        }
        println!("{}", A[s-1][t-1]);
    }
}