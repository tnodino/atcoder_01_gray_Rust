// https://atcoder.jp/contests/agc027/tasks/agc027_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        x: usize,
        mut a: [usize; N],
    }
    a.sort();
    let mut cnt = 0;
    let mut ans = 0;
    for i in 0..N {
        cnt += a[i];
        if cnt > x {
            break;
        }
        ans += 1;
    }
    if cnt < x {
        ans -= 1;
    }
    println!("{}", ans);
}