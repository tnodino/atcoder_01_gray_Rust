// https://atcoder.jp/contests/arc161/tasks/arc161_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    let M = N / 2 + 1;
    A.sort();
    for i in M..N {
        if A[i] == A[i-M+1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}