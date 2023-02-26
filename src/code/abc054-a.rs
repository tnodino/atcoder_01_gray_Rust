// https://atcoder.jp/contests/abc054/tasks/abc054_a

use proconio::input;
use proconio::fastout;

fn card(x: usize) -> usize {
    if x == 1 {
        return 14;
    }
    return x;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let A = card(A);
    let B = card(B);
    if A > B {
        println!("Alice");
    }
    else if A < B {
        println!("Bob");
    }
    else {
        println!("Draw");
    }
}