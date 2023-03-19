// https://atcoder.jp/contests/abc219/tasks/abc219_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
        N: usize,
        mut S: [String; N],
    }
    S.sort_by(|a, b| {
            let a = a.chars().collect::<Vec<char>>();
            let b = b.chars().collect::<Vec<char>>();
            let M = min(a.len(), b.len());
            for i in 0..M {
                if a[i] != b[i] {
                    return X.find(a[i]).cmp(&X.find(b[i]));
                }
            }
            return a.len().cmp(&b.len());
        }
    );
    for s in S {
        println!("{}", s);
    }
}