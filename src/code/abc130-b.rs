// https://atcoder.jp/contests/abc130/tasks/abc130_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        L: [usize; N],
    }
    let mut cnt = 0;
    let mut ans = 1;
    for l in L.iter() {
        cnt += l;
        if cnt <= X {
            ans += 1;
        }
        else {
            break;
        }
    }
    println!("{}", ans);
}