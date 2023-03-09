// https://atcoder.jp/contests/abc252/tasks/abc252_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; K],
    }
    let ma = A.iter().max().unwrap();
    for i in 0..N {
        if A[i] == *ma && B.contains(&(i+1)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}