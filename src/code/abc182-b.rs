// https://atcoder.jp/contests/abc182/tasks/abc182_b

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
    let mut gcd = 0;
    for a in 2..=1000 {
        let mut cnt = 0;
        for i in 0..N {
            if A[i] % a == 0 {
                cnt += 1;
            }
            if gcd < cnt {
                ans = a;
                gcd  = cnt;
            }
        }
    }
    println!("{}", ans);
}