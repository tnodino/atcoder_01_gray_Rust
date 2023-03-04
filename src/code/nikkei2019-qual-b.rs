// https://atcoder.jp/contests/nikkei2019-qual/tasks/nikkei2019_qual_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: String,
        B: String,
        C: String,
    }
    let A = A.chars().collect::<Vec<char>>();
    let B = B.chars().collect::<Vec<char>>();
    let C = C.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..N {
        let mut vec = vec![A[i], B[i], C[i]];
        vec.sort();
        vec.dedup();
        ans += vec.len() - 1;
    }
    println!("{}", ans);
}