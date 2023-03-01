// https://atcoder.jp/contests/abc097/tasks/abc097_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    if (a - c).abs() <= d {
        println!("Yes");
    }
    else if (a - b).abs() <= d && (b - c).abs() <= d {
        println!("Yes");
    }
    else {
        println!("No");
    }
}