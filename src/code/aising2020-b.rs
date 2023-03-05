// https://atcoder.jp/contests/aising2020/tasks/aising2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    for i in (0..N).step_by(2) {
        if a[i] % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}