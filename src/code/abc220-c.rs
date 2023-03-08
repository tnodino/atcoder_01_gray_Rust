// https://atcoder.jp/contests/abc220/tasks/abc220_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        X: usize,
    }
    let sum = A.iter().sum::<usize>();
    let mut ans = X / sum;
    let mut cnt = ans * sum;
    ans *= N;
    for a in A {
        ans += 1;
        cnt += a;
        if cnt > X {
            println!("{}", ans);
            return;
        }
    }
}