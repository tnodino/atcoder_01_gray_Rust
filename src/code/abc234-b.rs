// https://atcoder.jp/contests/abc234/tasks/abc234_b

use proconio::input;
use proconio::fastout;
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
    let mut ans = 0.;
    for i in 0..N {
        for j in 0..N {
            let dist = hypot((x[i] - x[j]).abs(), (y[i] - y[j]).abs());
            if ans < dist {
                ans = dist;
            }
        }
    }
    println!("{}", ans);
}