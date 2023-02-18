// https://atcoder.jp/contests/arc022/tasks/arc022_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    S = S.to_lowercase();
    let T = ['i', 'c', 't'];
    let mut idx = 0;
    let mut ans = "NO";
    for s in S.chars() {
        if T[idx] == s {
            idx += 1;
        }
        if idx == 3 {
            ans = "YES";
            break;
        }
    }
    println!("{}", ans);
}