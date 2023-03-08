// https://atcoder.jp/contests/abc148/tasks/abc148_d

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
    let mut cnt = 1;
    for v in a {
        if v == cnt {
            cnt += 1;
        }
        else {
            ans += 1;
        }
    }
    if cnt == 1 {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}