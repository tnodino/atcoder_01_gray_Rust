// https://atcoder.jp/contests/abc028/tasks/abc028_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut cnt = vec![0; 6];
    for s in S.chars() {
        let idx = s as usize - 65;
        cnt[idx] += 1;
    }
    let cnt = cnt.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    println!("{}", cnt.join(" "));
}