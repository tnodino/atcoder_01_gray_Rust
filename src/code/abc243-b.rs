// https://atcoder.jp/contests/abc243/tasks/abc243_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..N {
        for j in 0..N {
            if i == j && A[i] == B[j] {
                ans1 += 1;
            }
            if i != j && A[i] == B[j] {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);
}