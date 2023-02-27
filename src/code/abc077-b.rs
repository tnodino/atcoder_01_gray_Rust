// https://atcoder.jp/contests/abc077/tasks/abc077_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        if i * i > N {
            break;
        }
        ans = i * i;
    }
    println!("{}", ans);
}