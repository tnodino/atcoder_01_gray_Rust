// https://atcoder.jp/contests/code-festival-2015-quala/tasks/codefestival_2015_qualA_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans *= 2;
        ans += A[i];
    }
    println!("{}", ans);
}