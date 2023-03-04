// https://atcoder.jp/contests/abc205/tasks/abc205_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    for i in 1..=N {
        if !A.contains(&i) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}