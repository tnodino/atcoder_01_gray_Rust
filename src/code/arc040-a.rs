// https://atcoder.jp/contests/arc040/tasks/arc040_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut red = 0;
    let mut blue = 0;
    for _ in 0..N {
        input! {
            S: String,
        }
        for s in S.chars() {
            if s == 'R' {
                red += 1;
            }
            if s == 'B' {
                blue += 1;
            }
        }
    }
    if red > blue {
        println!("TAKAHASHI");
    }
    else if red < blue {
        println!("AOKI");
    }
    else {
        println!("DRAW");
    }
}