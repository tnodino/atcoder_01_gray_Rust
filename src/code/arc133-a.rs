// https://atcoder.jp/contests/arc133/tasks/arc133_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut x = -1;
    for i in 0..N-1 {
        if A[i] > A[i+1] {
            x = A[i];
            break;
        }
    }
    if x == -1 {
        x = A[N-1];
    }
    println!("{}", A.iter().filter(|&a| a != &x).map(|&a| a.to_string()).collect::<Vec<String>>().join(" "));
}