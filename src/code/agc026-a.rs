// https://atcoder.jp/contests/agc026/tasks/agc026_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut color = a[0];
    let mut cnt = 1;
    let mut ans = 0;
    for i in 1..N {
        if a[i] == color {
            cnt += 1;
        }
        else {
            ans += cnt / 2;
            color = a[i];
            cnt = 1;
        }
    }
    println!("{}", ans + cnt / 2);
}