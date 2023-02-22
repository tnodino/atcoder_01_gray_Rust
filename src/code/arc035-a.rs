// https://atcoder.jp/contests/arc035/tasks/arc035_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    let N = s.len();
    for i in 0..N/2 {
        let l = s[i];
        let r = s[N-i-1];
        if l != r && l != '*' && r != '*' {
            println!("NO");
            return;
        }
    }
    println!("YES");
}