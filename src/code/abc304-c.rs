// https://atcoder.jp/contests/abc304/tasks/abc304_c

use proconio::input;
use proconio::fastout;
use libm::hypot;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: f64,
    }
    let mut X = Vec::new();
    let mut Y = Vec::new();
    for _ in 0..N {
        input! {
            x: f64,
            y: f64,
        }
        X.push(x);
        Y.push(y);
    }
    let mut G = vec![Vec::new(); N];
    for i in 0..N {
        for j in i+1..N {
            if hypot((X[i] - X[j]).abs(), (Y[i] - Y[j]).abs()) <= D {
                G[i].push(j);
                G[j].push(i);
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back(0);
    let mut flg = vec![false; N];
    flg[0] = true;
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if flg[*nxt] {
                continue;
            }
            flg[*nxt] = true;
            que.push_back(*nxt);
        }
    }
    for i in 0..N {
        if flg[i] {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}