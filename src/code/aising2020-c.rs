// https://atcoder.jp/contests/aising2020/tasks/aising2020_c

use proconio::input;
use proconio::fastout;
use num::pow;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut ans = vec![0; N+1];
    for x in 1..M {
        for y in 1..M {
            for z in 1..M {
                let f = pow(x, 2) + pow(y, 2) + pow(z, 2) + x * y + x * z + y * z;
                if f <= N {
                    ans[f] += 1;
                }
            }
        }
    }
    for i in 1..=N {
        println!("{}", ans[i]);
    }
}