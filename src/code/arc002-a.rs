// https://atcoder.jp/contests/arc002/tasks/arc002_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Y: usize,
    }
    if Y % 400 == 0 {
        println!("YES");
    }
    else if Y % 100 == 0 {
        println!("NO");
    }
    else if Y % 4 == 0 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}