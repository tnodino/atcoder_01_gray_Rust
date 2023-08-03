// https://atcoder.jp/contests/abc310/tasks/abc310_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut P = vec![0; N];
    let mut flg = vec![vec![false; M]; N];
    for i in 0..N {
        input! {
            p: usize,
            C: usize,
            F: [usize; C],
        }
        P[i] = p;
        for j in 0..C {
            flg[i][F[j]-1] = true;
        }
    }
    for i in 0..N {
        'forj: for j in 0..N {
            if i == j {
                continue;
            }
            if P[i] < P[j] {
                continue;
            }
            for k in 0..M {
                if flg[i][k] && !flg[j][k] {
                    continue 'forj;
                }
            }
            if P[i] > P[j] {
                println!("Yes");
                return;
            }
            for k in 0..M {
                if !flg[i][k] && flg[j][k] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}