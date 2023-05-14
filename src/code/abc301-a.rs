// https://atcoder.jp/contests/abc301/tasks/abc301_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = S.iter().filter(|&x| *x == 'T').count();
    let A = N - T;
    if T > A {
        println!("T");
    }
    else if T < A {
        println!("A");
    }
    else {
        if S[N-1] == 'T' {
            println!("A");
        }
        else {
            println!("T");
        }
    }
}