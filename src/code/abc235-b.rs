// https://atcoder.jp/contests/abc235/tasks/abc235_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }
    let mut ans = H[0];
    for i in 1..N {
        if ans >= H[i] {
            break;
        }
        ans = H[i];
    }
    println!("{}", ans);
}