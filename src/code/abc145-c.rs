// https://atcoder.jp/contests/abc145/tasks/abc145_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..N {
        input! {
            a: f64,
            b: f64,
        }
        x.push(a);
        y.push(b);
    }
    let mut ans: f64 = 0.;
    let mut cnt: f64 = 0.;
    for perm in (0..N).permutations(N) {
        for i in 0..N-1 {
            let ax = x[perm[i]];
            let ay = y[perm[i]];
            let bx = x[perm[i+1]];
            let by = y[perm[i+1]];
            ans += hypot((ax - bx).abs(), (ay - by).abs());
        }
        cnt += 1.
    }
    println!("{}", ans / cnt);
}