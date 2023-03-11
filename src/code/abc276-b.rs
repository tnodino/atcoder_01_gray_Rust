// https://atcoder.jp/contests/abc276/tasks/abc276_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut ans = vec![vec![]; N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        ans[A-1].push(B);
        ans[B-1].push(A);
    }
    for i in 0..N {
        ans[i].sort();
        println!("{} {}", ans[i].len(), ans[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}