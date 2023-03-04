// https://atcoder.jp/contests/abc194/tasks/abc194_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A + B >= 15 && B >= 8 {
        println!("1");
    }
    else if A + B >= 10 && B >= 3 {
        println!("2");
    }
    else if A + B >= 3 {
        println!("3");
    }
    else {
        println!("4");
    }
}