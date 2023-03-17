// https://atcoder.jp/contests/abc293/tasks/abc293_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut flg = vec![true; N];
    for i in 0..N {
        if !flg[i] {
            continue;
        }
        flg[A[i]-1] = false;
    }
    let K = flg.iter().filter(|&x| *x).count();
    let mut X = Vec::new();
    for i in 0..N {
        if flg[i] {
            X.push(i+1);
        }
    }
    println!("{}\n{}", K, X.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}