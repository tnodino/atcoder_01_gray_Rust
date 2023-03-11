// https://atcoder.jp/contests/abc281/tasks/abc281_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut T: usize,
        A: [usize; N],
    }
    let sum = A.iter().sum::<usize>();
    T %= sum;
    for i in 0..N {
        if T < A[i] {
            println!("{} {}", i + 1, T);
            return;
        }
        else {
            T -= A[i];
        }
    }
}