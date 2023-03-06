// https://atcoder.jp/contests/abc227/tasks/abc227_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [usize; N],
    }
    let mut cnt = 0;
    for i in 0..N {
        '_break: for a in 1..=S[i] {
            for b in 1..=S[i] {
                if 4 * a * b + 3 * a + 3 * b == S[i] {
                    cnt += 1;
                    break '_break;
                }
            }
        }
    }
    println!("{}", N - cnt);
}