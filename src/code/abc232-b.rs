// https://atcoder.jp/contests/abc232/tasks/abc232_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    for k in 0..26 {
        let mut X = S.clone();
        for i in 0..N {
            let x = X[i] as u8 - 97;
            let x = (x + 26 - k) % 26;
            X[i] = (x + 97) as char;
        }
        if X == T {
            println!("Yes");
            return;
        }
    }
    println!("No");
}