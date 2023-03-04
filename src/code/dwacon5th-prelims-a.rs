// https://atcoder.jp/contests/dwacon5th-prelims/tasks/dwacon5th_prelims_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [f64; N],
    }
    let M = N as f64;
    let ave = a.iter().sum::<f64>() / M;
    let mut mi = (a[0] - ave).abs();
    let mut ans = 0;
    for (i, s) in a.iter().enumerate() {
        if (s - ave).abs() < mi {
            mi = (s - ave).abs();
            ans = i;
        }
    }
    println!("{}", ans);
}