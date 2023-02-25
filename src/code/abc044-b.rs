// https://atcoder.jp/contests/abc044/tasks/abc044_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        w: String,
    }
    let mut cnt = [0; 26];
    for x in w.chars() {
        let idx = x as usize - 97;
        cnt[idx] += 1;
    }
    for i in 0..26 {
        if cnt[i] % 2 == 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}