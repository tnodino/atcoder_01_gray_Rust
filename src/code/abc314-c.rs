// https://atcoder.jp/contests/abc314/tasks/abc314_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: String,
        C: [usize; N],
    }
    let mut S = S.chars().collect::<Vec<char>>();
    let mut vec = vec![Vec::new(); M+1];
    for i in 0..N {
        vec[C[i]].push(i);
    }
    for i in 1..=M {
        let m = vec[i].len();
        if m <= 1 {
            continue;
        }
        let tmp = S[vec[i][m-1]];
        for j in (0..m-1).rev() {
            S[vec[i][j+1]] = S[vec[i][j]];
        }
        S[vec[i][0]] = tmp;
    }
    println!("{}", S.iter().collect::<String>());
}