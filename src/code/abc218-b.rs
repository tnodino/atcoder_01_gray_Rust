// https://atcoder.jp/contests/abc218/tasks/abc218_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        P: [usize; 26],
    }
    let mut alp = ['?'; 26];
    for i in 0..26 {
        alp[i] = (P[i] as u8 + 96) as char;
    }
    println!("{}", alp.iter().map(|x| x.to_string()).collect::<String>());
}