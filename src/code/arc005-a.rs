// https://atcoder.jp/contests/arc005/tasks/arc005_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut w: [String; N],
    }
    let ta = "takahashikun";
    let mut ans = 0;
    for i in 0..N {
        w[i] = w[i].to_lowercase();
        if w[i] == ta || w[i] == format!("{}.", ta) {
            ans += 1
        }
    }
    println!("{}", ans);
}