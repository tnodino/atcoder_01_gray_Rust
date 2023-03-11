// https://atcoder.jp/contests/abc225/tasks/abc225_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![vec![]; N];
    for _ in 0..N-1 {
        input! {
            a: usize,
            b: usize,
        }
        G[a-1].push(b);
        G[b-1].push(a);
    }
    for i in 0..N {
        if G[i].len() == N - 1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}