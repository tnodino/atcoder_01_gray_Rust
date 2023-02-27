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
    let array = [0, 0, 1, 0, 1, 2, 3, 0, 1, 0];
    let mut ans = 0;
    for i in 0..n {
        ans += array[a[i]];
    }
    println!("{}", ans);
}