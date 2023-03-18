// https://atcoder.jp/contests/abc255/tasks/abc255_b

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; K],
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
    let mut flg = vec![false; N];
    for i in 0..K {
        flg[A[i]-1] = true;
    }
    let mut dist = vec![999_999.; N];
    for i in 0..N {
        if flg[i] {
            continue;
        }
        for j in 0..N {
            if !flg[j] {
                continue;
            }
            let d = hypot((X[i] - X[j]).abs(), (Y[i] - Y[j]).abs());
            if dist[i] > d {
                dist[i] = d;
            }
        }
    }
    let mut ans = 0.;
    for i in 0..N {
        if !flg[i] && ans < dist[i] {
            ans = dist[i];
        }
    }
    println!("{}", ans);
}