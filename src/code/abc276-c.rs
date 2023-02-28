// https://atcoder.jp/contests/abc276/tasks/abc276_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut ans = vec![0; N];
    let mut M = 0;
    for i in (0..N).rev() {
        if P[i-1] > P[i] {
            M = i - 1;
            break;
        }
    }
    let mut vec = Vec::new();
    for i in M..N {
        vec.push(P[i]);
    }
    vec.sort_by(|a, b| b.cmp(a));
    for i in 0..N-M {
        if vec[i] == P[M] {
            ans[M] = vec[i+1];
            break;
        }
    }
    for i in 0..M {
        ans[i] = P[i];
    }
    let mut idx = 0;
    for i in M+1..N {
        if ans[M] == vec[idx] {
            idx += 1;
        }
        ans[i] = vec[idx];
        idx += 1;
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}