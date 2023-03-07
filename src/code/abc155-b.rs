// https://atcoder.jp/contests/abc155/tasks/abc155_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    for i in 0..N {
        if A[i] % 2 == 0 && A[i] % 3 != 0 && A[i] % 5 != 0 {
            println!("DENIED");
            return;
        }
    }
    println!("APPROVED");
}