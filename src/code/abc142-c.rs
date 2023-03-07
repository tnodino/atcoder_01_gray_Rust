// https://atcoder.jp/contests/abc142/tasks/abc142_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = vec![0; N];
    for i in 0..N {
        ans[A[i]-1] = i + 1;
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}