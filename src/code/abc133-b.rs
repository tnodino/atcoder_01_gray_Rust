// https://atcoder.jp/contests/abc133/tasks/abc133_b

use proconio::input;
use proconio::fastout;
use num::pow;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
        X: [[f64; D]; N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            let mut dist = 0.;
            for d in 0..D {
                dist += pow(X[i][d] - X[j][d], 2);
            }
            if sqrt(dist) % 1. == 0. {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}