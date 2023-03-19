// https://atcoder.jp/contests/abc247/tasks/abc247_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut s = Vec::new();
    let mut t = Vec::new();
    for _ in 0..N {
        input! {
            x: String,
            y: String,
        }
        s.push(x);
        t.push(y);
    }
    for i in 0..N {
        let mut flg1 = false;
        let mut flg2 = false;
        for j in 0..N {
            if i == j {
                continue;
            }
            if s[i] == s[j] || s[i] == t[j] {
                flg1 = true;
            }
            if t[i] == s[j] || t[i] == t[j] {
                flg2 = true;
            }
        }
        if flg1 && flg2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}