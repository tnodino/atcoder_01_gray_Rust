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
    let mut flg = vec![false; N];
    for i in 0..N {
        if flg[i] {
            continue;
        }
        flg[A[i]-1] = true;
    }
    let vec = flg.iter().enumerate().filter(|&(_, x)| !*x).map(|(i, _)| i+1).collect::<Vec<usize>>();
    println!("{}", vec.len());
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}