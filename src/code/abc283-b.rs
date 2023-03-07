// https://atcoder.jp/contests/abc283/tasks/abc283_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                k: usize,
                x: usize,
            }
            A[k-1] = x;
        }
        else {
            input! {
                k: usize,
            }
            println!("{}", A[k-1]);
        }
    }
}