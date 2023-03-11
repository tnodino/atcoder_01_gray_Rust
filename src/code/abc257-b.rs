// https://atcoder.jp/contests/abc257/tasks/abc257_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        Q: usize,
        mut A: [usize; K],
        L: [usize; Q],
    }
    for i in 0..Q {
        if L[i] == K {
            if A[K-1] < N {
                A[K-1] += 1;
            }
        }
        else {
            let idx = L[i] - 1;
            if A[idx] + 1 < A[idx+1] {
                A[idx] += 1;
            }
        }
    }
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}