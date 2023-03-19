// https://atcoder.jp/contests/abc246/tasks/abc246_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
        X: usize,
        mut A: [usize; N],
    }
    for i in 0..N {
        if A[i] / X >= K {
            A[i] -= X * K;
            K = 0;
            break;
        }
        K -= A[i] / X;
        A[i] -= A[i] / X * X;
    }
    A.sort_by(|a, b| b.cmp(a));
    for i in 0..N {
        if K == 0 {
            break;
        }
        K -= 1;
        A[i] = 0;
    }
    println!("{}", A.iter().sum::<usize>());
}