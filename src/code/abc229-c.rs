// https://atcoder.jp/contests/abc229/tasks/abc229_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut W: usize,
        mut AB: [(usize, usize); N],
    }
    AB.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ans = 0;
    for i in 0..N {
        if W <= AB[i].1 {
            ans += AB[i].0 * W;
            break;
        }
        else {
            ans += AB[i].0 * AB[i].1;
            W -= AB[i].1;
        }
    }
    println!("{}", ans);
}