// https://atcoder.jp/contests/abc308/tasks/abc308_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        C: [String; N],
        D: [String; M],
        P: [usize; M+1],
    }
    let mut ans = 0;
    'outer: for i in 0..N {
        for j in 0..M {
            if C[i] == D[j] {
                ans += P[j+1];
                continue 'outer;
            }
        }
        ans += P[0];
    }
    println!("{}", ans);
}