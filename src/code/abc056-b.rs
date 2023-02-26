// https://atcoder.jp/contests/abc056/tasks/abc056_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        W: isize,
        a: isize,
        b: isize,
    }
    if (a - b).abs() <= W {
        println!("0");
    }
    else {
        println!("{}", (a - b).abs() - W);
    }
}