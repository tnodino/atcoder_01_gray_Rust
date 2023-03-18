// https://atcoder.jp/contests/arc123/tasks/arc123_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        mut B: [usize; N],
        mut C: [usize; N],
    }
    A.sort();
    B.sort();
    C.sort();
    let mut bidx = 0;
    let mut cidx = 0;
    let mut ans = 0;
    for i in 0..N {
        while bidx < N && A[i] >= B[bidx] {
            bidx += 1;
        }
        if bidx == N {
            break;
        }
        while cidx < N && B[bidx] >= C[cidx] {
            cidx += 1;
        }
        if cidx == N {
            break;
        }
        ans += 1;
        bidx += 1;
        cidx += 1;
    }
    println!("{}", ans);
}