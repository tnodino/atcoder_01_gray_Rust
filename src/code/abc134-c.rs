// https://atcoder.jp/contests/abc134/tasks/abc134_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut B = A.clone();
    B.sort_by(|a, b| b.cmp(a));
    let ma1 = B[0];
    let ma2 = B[1];
    for i in 0..N {
        if ma1 == A[i] {
            println!("{}", ma2);
        }
        else {
            println!("{}", ma1);
        }
    }
}