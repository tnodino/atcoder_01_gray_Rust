// https://atcoder.jp/contests/abc059/tasks/abc059_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        B: String,
    }
    let A = A.chars().collect::<Vec<char>>();
    let B = B.chars().collect::<Vec<char>>();
    if A == B {
        println!("EQUAL");
    }
    else if A.len() > B.len() {
        println!("GREATER");
    }
    else if A.len() < B.len() {
        println!("LESS");
    }
    else {
        for i in 0..A.len() {
            if A[i] > B[i] {
                println!("GREATER");
                return;
            }
            if A[i] < B[i] {
                println!("LESS");
                return;
            }
        }
    }
}