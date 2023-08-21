// https://atcoder.jp/contests/abc315/tasks/abc315_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    let alp = "aiueo".chars().collect::<Vec<char>>();
    for i in 0..5 {
        S = S.replace(alp[i], "");
    }
    println!("{}", S);
}