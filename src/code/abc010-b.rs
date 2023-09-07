// https://atcoder.jp/contests/abc010/tasks/abc010_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let m = 10;
    let mut flg = vec![false; m];
    for i in 1..m {
        if i % 2 != 0 && i % 3 != 2 {
            flg[i] = true;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (1..=a[i]).rev() {
            if flg[j] {
                ans += a[i] - j;
                break;
            }
        }
    }
    println!("{}", ans);
}