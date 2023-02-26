// https://atcoder.jp/contests/abc062/tasks/abc062_a

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn group(x: usize) -> usize {
    let A = [1, 3, 5, 7, 8, 10, 12];
    let B = [4, 6, 9, 11];
    if A.contains(&x) {
        return 1;
    }
    if B.contains(&x) {
        return 2;
    }
    return 3;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    if group(x) == group(y) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}