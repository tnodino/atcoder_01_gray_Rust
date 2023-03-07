// https://atcoder.jp/contests/abc143/tasks/abc143_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        d: [usize; N],
    }
    let mut ans = 0;
    let mut sum = d.iter().sum::<usize>();
    for i in 0..N {
        sum -= d[i];
        ans += sum * d[i];
    }
    println!("{}", ans);
}