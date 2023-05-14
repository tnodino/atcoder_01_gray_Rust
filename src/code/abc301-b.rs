// https://atcoder.jp/contests/abc301/tasks/abc301_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut vec = Vec::new();
    for i in 0..N-1 {
        if A[i] < A[i+1] {
            for a in A[i]..A[i+1] {
                vec.push(a);
            }
        }
        else if A[i] > A[i+1] {
            for a in (A[i+1]+1..=A[i]).rev() {
                vec.push(a);
            }
        }
        else {
            vec.push(A[i]);
        }
    }
    vec.push(A[N-1]);
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}