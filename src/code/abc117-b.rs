// https://atcoder.jp/contests/abc117/tasks/abc117_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut L: [usize; N],
    }
    L.sort();
    let M = L.iter().sum::<usize>() - L[N-1];
    if L[N-1] < M {
        println!("Yes");
    }
    else {
        println!("No");
    }
}