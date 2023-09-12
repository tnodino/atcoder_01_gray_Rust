// https://atcoder.jp/contests/abc004/tasks/abc004_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 4;
    input! {
        c: [[char; N]; N],
    }
    let mut ans = vec![vec!['?'; N]; N];
    for i in 0..N {
        for j in 0..N {
            ans[N-i-1][N-j-1] = c[i][j];
        }
    }
    for i in 0..N {
        println!("{}", ans[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}