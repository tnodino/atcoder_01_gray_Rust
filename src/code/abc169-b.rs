// https://atcoder.jp/contests/abc169/tasks/abc169_b

use proconio::input;
use proconio::fastout;

const MAX: usize = 1_000_000_000_000_000_000;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort();
    if A[0] == 0 {
        println!("0");
        return;
    }
    let mut ans = 1;
    for i in 0..N {
        if A[i] > MAX / ans {
            println!("-1");
            return;
        }
        ans *= A[i];
    }
    println!("{}", ans);
}