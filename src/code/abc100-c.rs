// https://atcoder.jp/contests/abc100/tasks/abc100_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        while a[i] % 2 == 0 {
            a[i] /= 2;
            ans += 1;
        }
    }
    println!("{}", ans);
}