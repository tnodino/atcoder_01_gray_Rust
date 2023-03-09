// https://atcoder.jp/contests/abc149/tasks/abc149_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize,
    }
    loop {
        for i in 2..=X {
            if i == X {
                println!("{}", X);
                return;
            }
            if X % i == 0 {
                X += 1;
                break;
            }
        }
    }
}