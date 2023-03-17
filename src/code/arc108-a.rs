// https://atcoder.jp/contests/arc108/tasks/arc108_a

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
        P: usize,
    }
    let N = sqrt(P as f64) as usize;
    for n in 1..=N {
        if P % n == 0 {
            let m = P / n;
            if n + m == S {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}