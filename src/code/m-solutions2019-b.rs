// https://atcoder.jp/contests/m-solutions2019/tasks/m_solutions2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if 15 - S.len() + S.chars().filter(|&x| x == 'o').count() >= 8 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}