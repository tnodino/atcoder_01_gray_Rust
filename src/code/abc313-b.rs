// https://atcoder.jp/contests/abc313/tasks/abc313_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![true; N];
    for _ in 0..M {
        input! {
            _A: usize,
            B: usize,
        }
        flg[B-1] = false;
    }
    let mut ans = Vec::new();
    for i in 0..N {
        if flg[i] {
            ans.push(i+1);
        }
    }
    if ans.len() == 1 {
        println!("{}", ans[0]);
    }
    else {
        println!("-1");
    }
}