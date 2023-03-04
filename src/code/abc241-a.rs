// https://atcoder.jp/contests/abc241/tasks/abc241_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: [usize; 10],
    }
    let mut idx = 0;
    for _ in 0..3 {
        idx = a[idx];
    }
    println!("{}", idx);
}