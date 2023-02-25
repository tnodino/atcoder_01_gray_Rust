// https://atcoder.jp/contests/abc036/tasks/abc036_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        s: [String; N],
    }
    let s: Vec<Vec<char>> = s.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
    let mut ans = vec![vec!['?'; N]; N];
    for i in 0..N {
        for j in 0..N {
            ans[i][j] = s[N-j-1][i];
        }
    }
    for i in 0..N {
        println!("{}", ans[i].iter().collect::<String>());
    }
}