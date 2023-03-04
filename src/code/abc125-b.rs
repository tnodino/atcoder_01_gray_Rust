// https://atcoder.jp/contests/abc125/tasks/abc125_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        V: [usize; N],
        C: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if V[i] > C[i] {
            ans += V[i] - C[i];
        }
    }
    println!("{}", ans);
}