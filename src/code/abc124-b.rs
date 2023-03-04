// https://atcoder.jp/contests/abc124/tasks/abc124_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }
    let mut ma = 0;
    let mut ans = 0;
    for i in 0..N {
        if H[i] >= ma {
            ma = H[i];
            ans += 1;
        }
    }
    println!("{}", ans);
}