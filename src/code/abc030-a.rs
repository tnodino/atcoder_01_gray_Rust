// https://atcoder.jp/contests/abc030/tasks/abc030_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
        C: f64,
        D: f64,
    }
    let ta = B / A;
    let ao = D / C;
    if ta > ao {
        println!("TAKAHASHI");
    }
    else if ta < ao {
        println!("AOKI");
    }
    else {
        println!("DRAW");
    }
}