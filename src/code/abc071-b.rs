// https://atcoder.jp/contests/abc071/tasks/abc071_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut flg = [true; 26];
    for s in S.chars() {
        let idx = s as usize - 97;
        flg[idx] = false;
    }
    for i in 0..26 {
        if flg[i] {
            println!("{}", (i as u8 + 97) as char);
            return;
        }
    }
    println!("None");
}