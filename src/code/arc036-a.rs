// https://atcoder.jp/contests/arc036/tasks/arc036_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        t: [usize; N],
    }
    let mut sleep = t[0] + t[1];
    for i in 2..N {
        sleep += t[i];
        if sleep < K {
            println!("{}", i + 1);
            return;
        }
        sleep -= t[i-2];
    }
    println!("-1");
}