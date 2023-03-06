// https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        D: usize,
    }
    let mut ans = 0;
    for m in 1..=M {
        for d in 1..=D {
            let d1 = d % 10;
            let d10 = d / 10;
            if d1 >= 2 && d10 >= 2 && d1 * d10 == m {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}