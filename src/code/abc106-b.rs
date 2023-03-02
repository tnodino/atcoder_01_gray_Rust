// https://atcoder.jp/contests/abc106/tasks/abc106_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for i in (3..=N).step_by(2) {
        let mut cnt = 0;
        for k in 1..=i {
            if i % k == 0 {
                cnt += 1;
            }
        }
        if cnt == 8 {
            ans += 1;
        }
    }
    println!("{}", ans);
}