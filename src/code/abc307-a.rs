// https://atcoder.jp/contests/abc307/tasks/abc307_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = vec![0; N];
    for i in 0..N {
        input! {
            A: [usize; 7],
        }
        ans[i] = A.iter().sum::<usize>();
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}