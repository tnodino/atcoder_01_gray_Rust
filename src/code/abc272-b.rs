// https://atcoder.jp/contests/abc272/tasks/abc272_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![vec![false; N]; N];
    for _ in 0..M {
        input! {
            k: usize,
            x: [usize; k],
        }
        for i in 0..k {
            for j in i+1..k {
                flg[x[i]-1][x[j]-1] = true;
                flg[x[i]-1][x[j]-1] = true;
            }
        }
    }
    for i in 0..N {
        for j in i+1..N {
            if !flg[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes")
}