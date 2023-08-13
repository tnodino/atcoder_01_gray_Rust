// https://atcoder.jp/contests/abc314/tasks/abc314_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut C = Vec::new();
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            c: usize,
            a: [usize; c],
        }
        C.push(c);
        A.push(a);
    }
    input! {
        X: usize,
    }
    let mut mi = 100;
    let mut vec = Vec::new();
    for i in 0..N {
        for j in 0..C[i] {
            if A[i][j] == X {
                mi = min(mi, C[i]);
                vec.push(i);
                break;
            }
        }
    }
    let mut ans = Vec::new();
    for i in 0..vec.len() {
        if C[vec[i]] == mi {
            ans.push(vec[i] + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}