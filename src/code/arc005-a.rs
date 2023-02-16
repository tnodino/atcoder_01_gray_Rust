// https://atcoder.jp/contests/arc005/tasks/arc005_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        w: [String; N],
    }
    let mut ans = 0;
    let TA = "TAKAHASHIKUN";
    let Ta = "Takahashikun";
    let ta = "takahashikun";
    for i in 0..N {
        if w[i] == TA || w[i] == Ta || w[i] == ta {
            ans += 1
        }
        if w[i] == TA.to_string() + "." || w[i] == Ta.to_string() + "." || w[i] == ta.to_string() + "." {
            ans += 1
        }
    }
    println!("{}", ans);
}