// https://atcoder.jp/contests/abc087/tasks/abc087_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        X: usize,
    }
    let mut ans = 0;
    for i in 0..=A {
        for j in 0..=B {
            for k in 0..=C {
                if 500 * i + 100 * j + 50 * k == X {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}